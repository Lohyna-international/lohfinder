#ifndef COMMANDS_DOMAIN_H
#define COMMANDS_DOMAIN_H

#include <string>

namespace eas::commands {
struct CreateForm {
  std::string data;
};

struct UpdateForm {
  std::string data;
};

struct DeleteFormAndResponses {
  std::string data;
};

struct CreateResponse {
  std::string data;
};

struct UpdateRespone {
  std::string data;
};

struct DeleteResponse {
  std::string data;
};

struct DeleteUserResponses {
  std::string data;
};
}  // namespace eas::commands

#endif  // COMMANDS_DOMAIN_H