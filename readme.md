# Scoregate 🦀

**A Rust-powered microservice that orchestrates digital lending decisions via secure KYC, score polling, and real-time loan lifecycle management.**

---

## ✨ Features Implemented

- [x] `POST /subscribe`  
  - Validates input  
  - Enforces one pending loan per customer  
  - Returns `201 Created` with `{ loan_id, state }`  
- [x] RESTful APIs for Loan Request and Status Tracking  
- [x] Secure retry-based integration with scoring engine (token + polling)  
- [ ] Transaction-data middleware endpoint  
- [ ] Mocked SOAP integration for KYC & transactions  
- [ ] Dockerized deployment  
- [ ] `.env`-driven config via `dotenvy`  
- [ ] Logging and error tracing using `tracing`  
- [ ] Modular code layout (`models`, `routes`, `services`, `auth`, `error`)  
- [ ] Screencast walkthrough  

---

## 🛠 Technologies

| Layer               | Tech              |
|---------------------|-------------------|
| Language            | Rust 2021         |
| Web Framework       | Warp 0.3          |
| Async Runtime       | Tokio             |
| In-Memory Store     | DashMap           |
| Data Modeling       | Serde, UUID       |
| Logging             | Tracing           |
| HTTP Client         | Reqwest           |
| XML Parsing         | quick-xml         |
| Retry Logic         | tokio-retry       |
| Config              | dotenvy           |
| Packaging           | Docker            |
