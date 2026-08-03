# Task Flow API Documentation

## 1. Project Overview

Task Flow API is a backend service built using Rust, Axum, SQLx, and PostgreSQL.

The purpose of the project is to provide a foundation for a healthcare/hospital management system with role-based access, patient management, visits, authentication, and secure patient record access.

## 2. Technology Stack

### Backend

* Rust
* Axum Web Framework
* Tokio Async Runtime

### Database

* PostgreSQL 17
* SQLx ORM
* SQLx Offline Mode for Docker builds

### Authentication

* JWT Authentication
* OTP-based Patient Login
* Password Hashing using bcrypt

### Deployment

* Docker
* Docker Compose

---

# Current Architecture

```
Client
 |
 |
Axum API
 |
 |-----------------
 |                |
Services       Middleware
 |
Repositories
 |
SQLx
 |
PostgreSQL
```

The project follows a layered architecture:

## Handlers Layer

Responsible for:

* Receiving HTTP requests
* Validating input
* Returning HTTP responses

Example:

```
src/handlers/
```

---

## Services Layer

Responsible for:

* Business logic
* Authentication flow
* OTP generation
* Data processing

Example:

```
src/services/
```

---

## Repository Layer

Responsible for:

* Database communication
* SQL queries
* PostgreSQL operations

Example:

```
src/repositories/
```

---

# Completed Features

## 1. Patient Management

Implemented:

* Create patient
* Get patient
* List patients
* Update patient
* Delete patient (future improvement)

Patient information includes:

* Name
* Age
* Gender
* Email
* Contact number
* Role

---

# 2. Visit Management

Implemented:

* Create visit
* Get visits
* Update visit
* Patient visit relationship

Current relationship:

```
Patient
   |
   |
 Visits
```

---

# 3. User Authentication

Implemented:

## User Registration

Users can be created with:

* Email
* Password
* Role

Passwords are stored as hashes.

---

## JWT Authentication

Implemented:

* JWT token creation
* JWT verification
* Protected routes

JWT contains:

* User ID
* Email
* Role
* Expiration time

---

# 4. Role System

Supported roles:

```
Admin
Doctor
Receptionist
Patient
```

Role-based authorization has been started.

Example:

```
Admin routes
Doctor routes
Receptionist routes
Patient routes
```

---

# 5. Patient OTP Login

Implemented patient login without passwords.

Flow:

```
Patient enters:

Email OR Phone Number

        |
        v

Backend generates OTP

        |
        v

OTP stored in database

        |
        v

Patient verifies OTP

        |
        v

JWT token returned
```

Current OTP delivery:

* Simulated
* Logged for testing

Future:

* SMS provider integration

---

# 6. Docker Deployment

Implemented:

Dockerfile:

* Rust build stage
* Runtime stage

Docker Compose:

```
task_flow_api
        |
        |
task_flow_postgres
```

PostgreSQL runs separately using Docker networking.

---

# Current API Routes

## Authentication

### Register User

```
POST /auth/register
```

Creates a new system user.

---

### Login

```
POST /auth/login
```

Returns JWT token.

---

## Patient Routes

Example:

```
GET /patients
```

Returns patients.

```
POST /patients
```

Creates patient.

```
PUT /patients/:id
```

Updates patient.

---

## Visit Routes

Example:

```
POST /visits
```

Creates a visit.

```
GET /visits
```

Gets visits.

---

## Patient OTP Routes

### Request OTP

```
POST /patient/request-otp
```

Input:

```
email OR contact
```

Output:

```
OTP generated
```

---

### Verify OTP

```
POST /patient/verify-otp
```

Input:

```
otp
```

Output:

```
JWT token
```

---

# Database Tables

Current tables:

```
users
patients
visits
patients_otp
```

Relationships:

```
users

patients

patients
   |
   |
 visits

patients
   |
   |
 patients_otp
```

---

# Remaining Features To Implement

## 1. Patient Profile Route

Create:

```
GET /patient/me
```

Purpose:

Allow logged-in patients to view:

* Personal information
* Visits
* Medical history
* Prescriptions
* Medication records

---

# 2. Prescription System

New table:

```
prescriptions
```

Fields:

```
id
visit_id
doctor_id
description
created_at
```

Relationship:

```
Visit
 |
 |
Prescription
```

---

# 3. Medication System

New table:

```
medications
```

Fields:

```
id
prescription_id
name
dosage
duration
```

Relationship:

```
Prescription
 |
 |
Medication
```

---

# 4. Doctor Features

Implement:

* Doctor login
* View assigned patients
* Create prescriptions
* Update medical notes

---

# 5. Receptionist Features

Implement:

* Create patient accounts
* Schedule visits
* Manage patient information

---

# 6. Admin Features

Implement:

* Create users
* Assign roles
* Manage permissions
* View system information

---

# 7. Security Improvements

Future improvements:

* Refresh tokens
* Better JWT expiration handling
* Rate limiting
* OTP attempt limits
* Password policies
* Audit logs

---

# 8. API Documentation

Future:

Add Swagger/OpenAPI documentation.

Recommended library:

```
utoipa
```

This will generate:

* Interactive API documentation
* Request examples
* Response schemas

---

# 9. Production Improvements

Before production:

* Better logging with tracing
* Monitoring
* Database backups
* HTTPS
* Environment secret management
* CI/CD pipeline
* Automated tests

---

# Current Status

The project currently has a working backend foundation:

✅ Rust API
✅ PostgreSQL database
✅ Docker deployment
✅ Repository architecture
✅ Authentication
✅ JWT
✅ OTP login
✅ Patient management
✅ Visit management

The next major milestone is completing the medical record system:

```
Patient
   |
   |
Visit
   |
   |
Prescription
   |
   |
Medication
```

After that, the system will be ready for a more complete hospital workflow.
