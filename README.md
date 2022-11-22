# Rust Practice

## Setup
### Create Network

```
docker network create rust-practice-network
```

## Usage

It is assumed that VS Code is used as the editor

### Reopen in Container

run [Reopen in Container]

### run

```
make watch
```
go to <http://localhost:8080/helth-check>

### migrate
```
make migrate-generate M={file_name} // ex) make migrate-generate M={create_users}
make migrate-run
```

### phpMyAdmin
go to <http://localhost:3002>
