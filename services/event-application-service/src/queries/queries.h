#ifndef QUERIES_H
#define QUERIES_H

#include "queries/domain.h"

namespace eas::queries {

struct IQueryHandler {
  virtual Form Execute(FormQuery) = 0;
  virtual Response Execute(UserResponseQuery) = 0;
  virtual FormResponses Execute(FormResponsesQuery) = 0;

  virtual ~IQueryHandler() {}
};

// Dummy query handler.
struct QueryHandler : public IQueryHandler {
  Form Execute(FormQuery) override { return {}; }
  Response Execute(UserResponseQuery) override { return {}; }
  FormResponses Execute(FormResponsesQuery) override { return {}; }
};

}  // namespace eas::queries

#endif  // QUERIES_H
