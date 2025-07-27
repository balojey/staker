# Section 16: Testing Strategy

## Testing Pyramid

We will follow the testing pyramid model, with a wide base of Vitest/RTL unit tests, a layer of integration tests, and focused Playwright E2E tests.

## Test Organization

  * **Frontend:** Tests will be co-located with the components they test.
  * **Backend:** Tests will use the existing Mocha/Chai setup in the `programs/` directory.
  * **E2E:** Tests will live in a root `e2e/` directory.
