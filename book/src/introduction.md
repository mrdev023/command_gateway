# Introduction

Hello, and welcome to the [PROJECT_NAME] project.
The goal of this project is to offer simple gateway to run script as command added by the DevOps.
The project is composed of two principal elements.

### 1. Command interpreter

Interpreter is the user main interface can be accessed like BASH.
It can be configured as default login user like that: 

```shell
useradd -s /path/to/interpreter [USER]
```

or

```shell
chsh [USER] /path/to/interpreter
```

### 2. Daemon

- It's used to manage interpreter session (Kill session, list, authenticate, ....).
- Allow external program to communicate with UNIX socket like external API or CLI.
- Allow use HTTP or UNIX Socket external authenticator.
It can be useful for integrate Oauth, OpenID, LDAP, ... from any language support GRPC protocol from protobuf.



