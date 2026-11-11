## Workflow of the Appointment Service 
```
        create
          ↓
      Scheduled ──────────┐
          ↓               │
      Confirmed           │
       ↙      ↘           ↓
   NoShow   Canceled ← (any stage)
     ↓
  Completed
```

## Workflow of Rescheudling an appointment
```
Patient clicks "Reschedule"
        ↓
Frontend calls GET /doctor/{id}/schedule
        ↓
Shows patient available slots to pick from
        ↓
Patient selects slot_003 (Monday 10:00am)
        ↓
Frontend sends PATCH /appointments/{id}/reschedule
  { "new_slot_id": "slot_003" }
        ↓
Backend validates and books it
```

| Actor | Operation |
|---|---|
| Patient | Create appointment (Can only choose appointment based on Doctor's schedule) |
| Patient | Reschedule appointment (Can only ReSchedule appointment based on Doctor's schedule) |
| Doctor | Confirm appointment (Doctor confirms a Schedule appointment) |
| Doctor | Cancel appointment (Can cancel appointment based on any reason) |
| Doctor | Complete appointment (Doctor marks complete if Patient shows up for appointment) |
| Doctor | Mark no-show (Doctor marks NoShow if Patient does not show up for appointment) |