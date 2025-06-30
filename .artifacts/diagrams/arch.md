---
title: Architecture
description: This document provides an overview of the architecture of the ACME Engine, detailing its components and their interactions.
tagline: A technical review of the acme platform architecture
date: 2025-06-30 10:00:00 +0000
tags: [architecture, acme, engine, automation, orchestration]
author: FL03
nav_order: 2
---

## Architecture

The `acme` platform focuses on providing a secure, robust, and efficient synchronization utility that aggregates and processes data from various sources. The architecture is designed to be modular and extensible, allowing for easy integration of new data sources and sinks.

### _Diagram 1_: Engine Architecture

This diagram illustrates the architecture of the ACME Engine, which is designed to automate and orchestrate tasks across various sources and sinks. The engine consists of several key components that work together to process data from different sources, normalize it, and store it for further use. The architecture is modular, allowing for extensibility through plugins and custom sources.

```mermaid
flowchart TD
    subgraph Engine
        Scheduler["⏰ Scheduler"]
        Dispatcher["🚀 Dispatcher"]
        PipeRouter["🔀 Pipe Router"]
        Normalizer["🔧 Normalizer"]
        DataStore["🧠 In-Memory/Data Store"]
        PluginHost["🔌 Plugin Host"]
    end

    subgraph Sources
        API[API Sources]
        File[File/Log Sources]
        Custom[Custom/Scripted Sources]
    end

    API -->|REST/GraphQL Polling| Dispatcher
    File -->|File Watchers| Dispatcher
    Custom -->|Plugin/IPC| PluginHost

    Dispatcher -->|Dispatch Task| PipeRouter
    PluginHost -->|Plugin Result| PipeRouter
    Scheduler -->|Timed Trigger| Dispatcher

    PipeRouter -->|Route + Normalize| Normalizer
    Normalizer -->|Emit JSON| DataStore

    DataStore -->|Frontend API| UI[📊 Frontend Dashboard]
```
