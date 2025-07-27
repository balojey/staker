# Section 18: Error Handling Strategy

A central `ErrorParsingService` will catch raw blockchain errors and transform them into standardized, user-friendly messages displayed in a Toast notification. The service will map known on-chain program error codes to human-readable text.
