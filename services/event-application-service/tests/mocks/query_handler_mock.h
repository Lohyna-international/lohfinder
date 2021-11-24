#ifndef MOCKS_QUERY_HANDLER_MOCK_H
#define MOCKS_QUERY_HANDLER_MOCK_H

#include <gmock/gmock.h>

#include "queries/queries.h"

class QueryHandlerMock : public eas::queries::IQueryHandler {
 public:
  MOCK_METHOD(eas::queries::Form, Execute, (eas::queries::FormQuery),
              (override));
  MOCK_METHOD(eas::queries::Response, Execute,
              (eas::queries::UserResponseQuery), (override));
  MOCK_METHOD(eas::queries::FormResponses, Execute,
              (eas::queries::FormResponsesQuery), (override));
};

#endif  // MOCKS_QUERY_HANDLER_MOCK_H