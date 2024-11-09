# Backlog: Lerd Project

## Phase 1: Basic Setup and Service Management

- [x] **Project Setup with Tauri**
  - [x] Install Tauri and configure the project for the frontend and CLI interface.
  - [x] Initialize the project with a standard folder structure and basic settings.

- [ ] **Basic Service Management**
  - [ ] Develop the `Service Manager` to handle services such as PHP, NGINX, MySQL, Node.js, Redis, MongoDB.
  - [ ] Add CLI commands `lerd start` and `lerd stop` to start and stop individual services from the command line.
  - [ ] Test compatibility and performance on both Windows and Ubuntu.

- [ ] **Design YAML Configuration Structure**
  - [ ] Create a `~/.lerd-config` directory to store service configurations.
  - [ ] Create sample configuration files for each service (`php.yaml`, `mysql.yaml`, `nodejs.yaml`, `services.yaml`) to store version and service status.

- [ ] **Basic Logging Integration**
  - [ ] Create a `~/.lerd-logs` directory for storing logs.
  - [ ] Set up logging for each action (start/stop) for individual services.

---

## Phase 2: Local Domain `.test` Management and SSL

- [ ] **Local `.test` Domain Management**
  - [ ] Add functionality to allow users to add/remove `.test` domains.
  - [ ] Automatically create a Virtual Host file in NGINX for each `.test` domain.

- [ ] **Self-Signed SSL Support**
  - [ ] Integrate OpenSSL to generate self-signed SSL certificates for `.test` domains with SSL enabled.
  - [ ] Store SSL certificates in the `~/.lerd-ssl` directory.
  - [ ] Update NGINX configuration for each domain to use self-signed SSL certificates when needed.

- [ ] **Domain Management Interface**
  - [ ] Build an interface allowing users to add/remove domains, enable/disable SSL, and set the root directory for each domain.
  - [ ] Integrate the `lerd link` CLI to allow users to link the root directory automatically to the domain name.

---

## Phase 3: Installing and Switching PHP, MySQL, and Node.js Versions from Zip Files

- [ ] **Download and Install PHP from Zip Files**
  - [ ] Create a `~/.lerd-php` directory to store downloaded PHP versions.
  - [ ] Implement a function to download PHP zip files from a specified URL and extract them into `.lerd-php`.
  - [ ] Update the CLI `lerd php <version>` to switch the PHP version by adding the path of the version to the `PATH` environment variable.

- [ ] **Download and Install MySQL from Zip Files**
  - [ ] Create a `~/.lerd-mysql` directory to store downloaded MySQL versions.
  - [ ] Implement a function to download MySQL zip files from a specified URL and extract them into `.lerd-mysql`.
  - [ ] Update the CLI `lerd mysql <version>` to switch the MySQL version.

- [ ] **Manage Node.js Versions with `nvm`**
  - [ ] Support Node.js version management by using `nvm`.
  - [ ] Create the CLI command `lerd node <version>` to switch Node.js versions via `nvm use`.

- [ ] **Save Configuration for Service Versions**
  - [ ] Update YAML configuration files (`php.yaml`, `mysql.yaml`, `nodejs.yaml`) to store the current version of each service.
  - [ ] Verify and ensure that versions are accurately updated in the configurations.

---

## Phase 4: Testing, Documentation, and Release

- [ ] **Testing**
  - [ ] Conduct comprehensive feature testing on both Windows and Ubuntu.
  - [ ] Ensure Service Manager and CLI commands are compatible across platforms.
  - [ ] Optimize performance and test error handling.

- [ ] **Documentation**
  - [ ] Write user documentation for the CLI and interface features.
  - [ ] Provide initial setup instructions, including adding domains, enabling SSL, and switching service versions.
  - [ ] Include configuration file (`YAML`) setup guidance.

- [ ] **Release**
  - [ ] Prepare the first release of "Lerd," including source code and setup instructions.
  - [ ] Package the application and publish it on GitHub or another platform for download.
  - [ ] Gather user feedback for planning future improvements.

---

## Phase 5: Extended Features and PostgreSQL Support (Optional)

- [ ] **PostgreSQL Support**
  - [ ] Integrate PostgreSQL into the Service Manager and update CLI to support installation and management.
  - [ ] Add PostgreSQL configuration to the YAML file and interface.

- [ ] **Additional Extended Features**
  - [ ] Add logs viewing for individual domains directly within the interface.
  - [ ] Implement automatic detection of newly installed services and update the interface.
  - [ ] Consider additional features such as IDE integrations or supplementary library management.

