(() => {
  const collection = db.getCollection("doctor_schedule_collection");
  const legacyGroups = collection.find({ slots: { $exists: true } }).toArray();
  const migratedSlots = [];
  const slotIds = new Set();

  function fail(message) {
    throw new Error(`Doctor schedule migration stopped: ${message}`);
  }

  function normalizeDate(value, label) {
    let date;
    if (value instanceof Date) {
      date = value;
    } else if (typeof value === "string") {
      const milliseconds = Date.parse(value);
      if (!Number.isNaN(milliseconds)) {
        date = new Date(milliseconds);
      }
    }

    if (!date || Number.isNaN(date.getTime())) {
      fail(`${label} must be a valid BSON date or date string`);
    }
    return date;
  }

  function sameDate(actual, expected) {
    return actual instanceof Date && actual.getTime() === expected.getTime();
  }

  function matchesSlot(existing, expected) {
    if (!existing || Object.prototype.hasOwnProperty.call(existing, "slots")) {
      return false;
    }

    for (const field of ["doctor_id", "is_available"]) {
      if (existing[field] !== expected[field]) {
        return false;
      }
    }
    for (const field of ["start_time", "end_time", "created_at"]) {
      if (!sameDate(existing[field], expected[field])) {
        return false;
      }
    }

    const hasExpectedUpdatedAt = Object.prototype.hasOwnProperty.call(
      expected,
      "updated_at"
    );
    const hasExistingUpdatedAt = Object.prototype.hasOwnProperty.call(
      existing,
      "updated_at"
    );
    if (hasExpectedUpdatedAt !== hasExistingUpdatedAt) {
      return false;
    }
    if (
      hasExpectedUpdatedAt &&
      expected.updated_at !== null &&
      !sameDate(existing.updated_at, expected.updated_at)
    ) {
      return false;
    }
    return !hasExpectedUpdatedAt || existing.updated_at === expected.updated_at ||
      sameDate(existing.updated_at, expected.updated_at);
  }

  for (const group of legacyGroups) {
    if (typeof group.doctor_id !== "string" || group.doctor_id.length === 0) {
      fail(`legacy group ${String(group._id)} has no valid doctor_id`);
    }
    if (!Array.isArray(group.slots) || group.slots.length === 0) {
      fail(`legacy group ${String(group._id)} has no schedule slots`);
    }

    for (let index = 0; index < group.slots.length; index += 1) {
      const slot = group.slots[index];
      const label = `legacy group ${String(group._id)} slot ${index}`;
      if (!slot || typeof slot !== "object" || Array.isArray(slot)) {
        fail(`${label} is not a document`);
      }

      if (
        slot._id !== undefined &&
        slot.slot_id !== undefined &&
        slot._id !== slot.slot_id
      ) {
        fail(`${label} has conflicting _id and slot_id values`);
      }
      const slotId = slot._id === undefined ? slot.slot_id : slot._id;
      if (typeof slotId !== "string" || slotId.length === 0) {
        fail(`${label} has no valid slot ID`);
      }
      if (slotIds.has(slotId)) {
        fail(`duplicate slot ID ${slotId}`);
      }
      slotIds.add(slotId);

      const startTime = normalizeDate(slot.start_time, `${label} start_time`);
      const endTime = slot.end_time === undefined
        ? new Date(startTime.getTime() + 30 * 60 * 1000)
        : normalizeDate(slot.end_time, `${label} end_time`);
      if (endTime.getTime() <= startTime.getTime()) {
        fail(`${label} end_time must be after start_time`);
      }
      if (slot.is_available !== undefined && typeof slot.is_available !== "boolean") {
        fail(`${label} is_available must be a boolean`);
      }
      const createdAt = normalizeDate(slot.created_at, `${label} created_at`);

      const migrated = {
        _id: slotId,
        doctor_id: group.doctor_id,
        start_time: startTime,
        end_time: endTime,
        is_available: slot.is_available === undefined ? false : slot.is_available,
        created_at: createdAt,
      };
      if (slot.updated_at !== undefined) {
        migrated.updated_at = slot.updated_at === null
          ? null
          : normalizeDate(slot.updated_at, `${label} updated_at`);
      }
      migratedSlots.push({ groupId: group._id, document: migrated });
    }
  }

  // Preflight every target ID before writing, so malformed/conflicting source data
  // never causes a legacy group to be removed or an existing slot to be replaced.
  for (const { document } of migratedSlots) {
    const existing = collection.findOne({ _id: document._id });
    if (existing && !matchesSlot(existing, document)) {
      fail(`slot ID ${document._id} already belongs to a different document`);
    }
  }

  collection.createIndex(
    { doctor_id: 1, start_time: 1, _id: 1 },
    { name: "doctor_schedule_lookup" }
  );

  for (const { document } of migratedSlots) {
    if (!collection.findOne({ _id: document._id })) {
      collection.insertOne(document);
    }
    const stored = collection.findOne({ _id: document._id });
    if (!matchesSlot(stored, document)) {
      fail(`slot ID ${document._id} could not be safely verified after insertion`);
    }
  }

  for (const group of legacyGroups) {
    const result = collection.deleteOne({
      _id: group._id,
      slots: { $exists: true },
    });
    if (result.deletedCount !== 1) {
      fail(`legacy group ${String(group._id)} was not removed after verification`);
    }
  }

  print(
    `Migrated ${migratedSlots.length} schedule slots from ${legacyGroups.length} legacy groups in ${db.getName()}`
  );
})();