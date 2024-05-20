# Image Processor

The project is a simple Image Processor that allows users to upload images, apply filters, and download the processed images. The project is divided into two main components:

-   API Services: A set of RESTful APIs built using [Rust](https://www.rust-lang.org/pt-BR) and [Actix](https://actix.rs/) for user authentication and image processing.
-   Dashboard: A mobile application template built using [Flutter](https://flutter.dev/) and [Rust](https://www.rust-lang.org/pt-BR) with bindings to the API services.

## API

The API services are managed by the [nginx](https://www.nginx.com/) web server, currently running on a [Docker](https://www.docker.com/) container. All the services has a log system that saves the logs in database.

### Auth Service

The Auth service is responsible for managing user authentication and authorization. It provides endpoints for user registration, login, and session management.

#### Endpoints

-   `POST /user/register`: Register a new user.
-   `POST /user/login`: Login a user.
-   `GET /user/`: Get the current user's information.

### Prerequisites

-   [Docker](https://docs.docker.com/get-docker/) for running the API services in a container.

### Installation and Running

#### Setup

1. Get the project:

    First, clone the repository:

    ```sh
    git@github.com:ViniciosLugli/image-processor.git
    ```

    Then, navigate to the project and API directory, where the server is located:

    ```sh
    cd image-processor/services
    ```

2. Set environment variables:

    Copy the `.env.example` file to `.env` and fill in the required values:

    ```sh
    cp .env.example .env
    ```

    Update the `.env` file with the required values if necessary:

    ```sh
    RUST_LOG=info
    DATABASE_URL="postgresql://postgres:postgres@localhost:5432/postgres?schema=public"
    REDIS_URL="redis://localhost:6379"
    ```

3. Run the services:

    Start the services with nginx using the following command:

    ```sh
    docker compose -f docker-compose-dev.yml up --builçd
    ```

    This docker compose also starts a postgres and redis container. If you want to start only the services, you can use the following command:

    ```sh
    docker compose -f docker-compose-prod.yml up --build services
    ```

### Usage

Once the server is running, it will listen on [localhost:3000](http://localhost:3000). You can interact with the API using any HTTP client.

#### API Endpoints

You can use the [Insomnia](https://insomnia.rest/) collection file [assets/insomnia_collection.json](assets/Insomnia.json) to test the API and see the available endpoints.
![image](https://github.com/ViniciosLugli/image-processor/assets/40807526/d3e63d5e-7494-40cd-9e6a-5d16a60792a2)

## Dashboard Component (`dashboard/`)

### Overview

A mobile application template in Flutter designed for the ToDo List project. It includes necessary setups for a streamlined development process with a focus on usability and adaptability for developers. The flutter app has bindings to the Rust calls using the `flutter_rust_bridge` plugin to interact with the API.

### Used Tools

-   [Flutter](https://flutter.dev/): A UI toolkit for building natively compiled applications for mobile, web, and desktop from a single codebase.
-   [flutter_rust_bridge](https://github.com/fzyzcjy/flutter_rust_bridge) A Flutter plugin for calling Rust functions from Dart.

### Architecture

The Flutter project is structured as a mobile application with the following components:

#### Components

The app is divided into two main components:

-   [dart sources](./dashboard/lib/): Initializes the Flutter app and routes requests to the appropriate screens.
-   [rust sources](./dashboard/rust/): Contains the Rust code that will be called from the Flutter app.

### Setup and Installation

Follow the detailed setup instructions in the [Flutter Mobile Template README](https://github.com/ViniciosLugli/flutter-mobile-template) to set up the Flutter project and configure the development environment with [Android Studio](https://developer.android.com/studio) and an Android emulator or physical device.

### Prerequisites

-   [Flutter SDK](https://flutter.dev/docs/get-started/install) (latest stable version)
-   [Android Studio](https://developer.android.com/studio) with an Android emulator or a physical device.
-   [API](./api) server running locally or in a container.
-   [Rust](https://www.rust-lang.org/pt-BR) (latest stable version)

### Running the Project

1. Navigate to the dashboard directory:

    ```sh
    cd image-processor/dashboard
    ```

2. Install dependencies and set up the environment:

    - Install Flutter dependencies: `flutter pub get`
    - Set up Android Studio and configure emulators.
    - Install the Rust bridge: `cargo install 'flutter_rust_bridge_codegen@^2.0.0-dev.33`

3. Set the environment variables:

    Copy the `.env.example` file to `.env` and fill in the required values:

    ```sh
    cp .env.example .env
    ```

    Update the `.env` file with the required values:

    ```sh
    API_URL = "http://YOUR_API_IP:3000"
    ```

    If you are running the API locally, you can use the command:

    ```sh
    hostname -I
    ```

    This command will return the IP address of your machine. Use this IP address in the `.env` file.

4. Launch and run the project on an emulator:
    1. Configure and launch an emulator: `flutter emulators --launch <device_name>`
    2. Run the project: `flutter_rust_bridge_codegen generate && flutter run`

## Demo

The following video demonstrates the current state of the project:

<div align="center">
  <video src="https://github.com/ViniciosLugli/image-processor/assets/40807526/61123783-8568-4245-b4a4-6fbf2d583a9b" />
</div>
