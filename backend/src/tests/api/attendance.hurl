# Create Attendance
POST http://localhost:8000/attendances/new
Content-Type: application/json

{
  "user_id": "665bdb8e7f9c64b42d987abc",
  "organization_id": "665bdb8e7f9c64b42d987def",
  "attendance_type": "singlemark",
  "status": "present",
  "clock_in": "2025-06-05T09:00:00Z",
  "clock_out": null,
  "method": "manual",
  "location": {
    "lat": -6.200000,
    "long": 106.816666
  }
}

# Get All Attendances
GET http://localhost:8000/attendances/all

# Get Specific Attendance by ID
GET http://localhost:8000/attendances/68414c3d0443963ef42d2773

# Update Attendance
PUT http://localhost:8000/attendances/68414c3d0443963ef42d2773
Content-Type: application/json

{
  "attendance_type": "doublemark",
  "status": "absent",
  "clock_in": null,
  "clock_out": "2025-06-05T17:00:00Z",
  "method": "manual",
  "location": {
    "lat": -6.210000,
    "long": 106.820000
  }
}

# Delete Attendance
DELETE http://localhost:8000/attendances/68414c3d0443963ef42d2773
