# Hospital Management System API

A backend hospital management system built with **Rust**, **Axum**, and **PostgreSQL**.
The system provides secure authentication, role-based access control, patient management, clinic visits, and medical record handling.

The project is designed with a clean architecture approach using the **Repository Pattern**, separating business logic, database operations, and API layers.

---

# Features

## Authentication & Authorization

- User registration and login
- Password hashing using bcrypt
- JWT-based authentication
- Role-based authorization middleware

Supported roles:

- Admin
- Receptionist
- Doctor
- Patient

---

## Patient Management

Receptionists can create and manage patient records.

Patient information includes:

- Personal details
- Contact information
- Medical history references
- Assigned role

Patients authenticate using an OTP-based login flow:

```
Patient enters email/contact
        ↓
System generates OTP
        ↓
OTP verification
        ↓
JWT session created
        ↓
Patient accesses personal dashboard
```

Patients do not require passwords.

---

## Visit Management

The system supports clinic visit management.

A visit represents an official hospital interaction between a patient and healthcare staff.

A visit can contain:

- Patient information
- Doctor information
- Diagnosis
- Treatment details
- Medical notes

---

## Patient Dashboard

Authenticated patients can access their medical information through a single endpoint.

Example:

```
GET /patient/me
```

The dashboard provides:

- Patient profile
- Visits history
- Prescriptions
- Medications
- Other medical records

The patient identity is extracted from the JWT token, preventing unauthorized access to other patient records.

---

# Architecture

The project follows a layered architecture:

```
src
│
├── controllers
│       API handlers
│
├── services
│       Business logic
│
├── repositories
│       Database operations
│
├── models
│       Database models
│
├── dto
│       Request/Response objects
│
├── middleware
│       Authentication and authorization
│
├── errors
│       Application error handling
│
└── db
        PostgreSQL connection and migrations
```

---

# Technology Stack

## Backend

- Rust
- Axum Web Framework
- Tokio Async Runtime
- SQLx Database Toolkit

## Database

- PostgreSQL

## Security

- JWT Authentication
- bcrypt Password Hashing
- Role-Based Access Control

## Deployment

- Docker
- Docker Compose

---

# Environment Variables

Create a `.env` file:

```env
APP_HOST=0.0.0.0
APP_PORT=4000

DATABASE_URL=postgres://username:password@localhost:5432/hospital_db

JWT_SECRET=your_secret_key

RUST_LOG=info
```

For sharing the project, use:

```
.env.example
```

and never commit real secrets.

---

# Running Locally

## Clone repository

```bash
git clone <repository-url>

cd hospital-management
```

---

## Start PostgreSQL

Using Docker:

```bash
docker compose up postgres
```

---

## Run migrations

```bash
sqlx migrate run
```

---

## Start application

```bash
cargo run
```

The API will start on:

```
http://localhost:4000
```

---

# Running with Docker

Build and start all services:

```bash
docker compose up --build
```

The API will be available at:

```
http://localhost:4000
```

---

# API Overview

## Authentication

### Register User

```
POST /auth/register
```

### Login

```
POST /auth/login
```

Returns JWT token.

---

## Patient

### Request OTP

```
POST /patient/request-otp
```

### Verify OTP

```
POST /patient/verify-otp
```

### Patient Dashboard

```
GET /patient/me
```

Requires:

```
Authorization: Bearer <token>
```

---

## Visits

### Create Visit

```
POST /visits
```

### Get Visits

```
GET /visits
```

### Get Visit

```
GET /visits/{id}
```

---

# Error Handling

The application uses a centralized error system.

Handled errors include:

- Unauthorized access
- Forbidden actions
- Resource not found
- Validation failures
- Database errors

---

# Future Improvements

Planned improvements:

- Frontend patient portal
- Doctor dashboard
- Appointment scheduling
- Prescription management
- Medical file uploads
- Notifications
- AI-assisted symptom analysis
- Production monitoring

---

# Project Goals

This project demonstrates building a production-style backend application in Rust including:

- Async programming
- REST API design
- Database architecture
- Authentication systems
- Middleware
- Docker deployment
- Clean code organization

---

# License

This project is for educational and portfolio purposes.
