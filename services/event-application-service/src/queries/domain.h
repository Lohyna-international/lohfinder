#ifndef QUERIES_DOMAIN_H
#define QUERIES_DOMAIN_H

#include <string>
#include <string_view>

namespace eas::queries {
struct FormQuery {
  std::string data;
};

struct UserResponseQuery {
  std::string data;
};

struct FormResponsesQuery {
  std::string data;
};

// publisher object.
struct Form {
  static constexpr std::string_view topic = "eas_form_get";
  std::string data;
  std::string ToString() const { return data; }
};

struct Response {
  static constexpr std::string_view topic = "eas_response_get";
  std::string data;
  std::string ToString() const { return data; }
};

struct FormResponses {
  static constexpr std::string_view topic = "eas_form_responses_get";
  std::string data;
  std::string ToString() const { return data; }
};
}  // namespace eas::queries

#endif  // QUERIES_DOMAIN_H