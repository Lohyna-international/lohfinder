#ifndef PUBSUB_COMMUNICATION_PUBSUB_SUBSCRIBERS_H
#define PUBSUB_COMMUNICATION_PUBSUB_SUBSCRIBERS_H

#include <string_view>

#include "google/cloud/pubsub/ack_handler.h"
#include "google/cloud/pubsub/message.h"

namespace eas::pubsub_app {

struct CreateFormSub {
  static constexpr std::string_view topic = "eas_form_create";
  void operator()(google::cloud::pubsub::Message const &msg,
                  google::cloud::pubsub::AckHandler ack);
};

struct UpdateFormSub {
  static constexpr std::string_view topic = "eas_form_update";
  void operator()(google::cloud::pubsub::Message const &msg,
                  google::cloud::pubsub::AckHandler ack);
};

struct DeleteFormAndResponsesSub {
  static constexpr std::string_view topic = "eas_form_and_responses_delete";
  void operator()(google::cloud::pubsub::Message const &msg,
                  google::cloud::pubsub::AckHandler ack);
};

struct GetFormSub {
  static constexpr std::string_view topic = "eas_form_get";
  void operator()(google::cloud::pubsub::Message const &msg,
                  google::cloud::pubsub::AckHandler ack);
};

struct CreateResponseSub {
  static constexpr std::string_view topic = "eas_response_create";
  void operator()(google::cloud::pubsub::Message const &msg,
                  google::cloud::pubsub::AckHandler ack);
};

struct DeleteResponseSub {
  static constexpr std::string_view topic = "eas_response_delete";
  void operator()(google::cloud::pubsub::Message const &msg,
                  google::cloud::pubsub::AckHandler ack);
};

struct GetResponseSub {
  static constexpr std::string_view topic = "eas_response_get";
  void operator()(google::cloud::pubsub::Message const &msg,
                  google::cloud::pubsub::AckHandler ack);
};

struct GetAllEventResponses {
  static constexpr std::string_view topic = "eas_form_responses_get";
  void operator()(google::cloud::pubsub::Message const &msg,
                  google::cloud::pubsub::AckHandler ack);
};

struct DeleteAllUserResponsesSub {
  static constexpr std::string_view topic = "eas_responses_user_delete";
  void operator()(google::cloud::pubsub::Message const &msg,
                  google::cloud::pubsub::AckHandler ack);
};

}  // namespace eas::pubsub_app

#endif  // PUBSUB_COMMUNICATION_PUBSUB_SUBSCRIBERS_H