# ACM ISCTE-IUL Website

Welcome to the official repository for the ACM ISCTE-IUL Website – the online hub for the ACM Student Chapter at ISCTE-IUL. This project not only powers our public website but also includes a custom dashboard for managing internal association activities, events, and member engagement.

## 🚀 Overview

The project is built using a modern tech stack with a clear separation between the frontend and backend, containerized deployment, and dedicated configuration for a scalable production environment.

## 📌 Features

- **Modern UI/UX**: A responsive and dynamic user interface built with Svelte and TypeScript.
- **Custom Dashboard**: An exclusive dashboard for association management with real-time analytics, member management, and event planning.
- **Event & Content Management**: Easily showcase events, share blog posts, and manage membership registrations.
- **Containerized Deployment**: Utilizes Docker and Docker Compose along with Nginx for efficient deployment.

## 🛠️ Tech Stack

- **Frontend**: Built with **Svelte** using **TypeScript** for a reactive and fast user experience.
- **Backend**: Developed in **Rust**, providing a robust, high-performance API.
- **Database**: Uses SurrealDB (with data stored in the `surrealdb-data` folder) for flexible, scalable data management.
- **Deployment & Configuration**:
  - **Dockerfile**: Defines the container image for the application.
  - **docker-compose.yml**: Orchestrates multi-container deployment.
  - **Nginx**: Configuration files (`nginx.conf`) are provided to manage reverse-proxy and static file serving.


## ⚙️ Setup & Installation

### Prerequisites
- [Docker](https://www.docker.com/get-started) installed on your system
### Running Locally with Docker

1. **Clone the repository:**
   ```bash
   git clone https://github.com/npcnuno/acmweb.git
   cd acmweb
