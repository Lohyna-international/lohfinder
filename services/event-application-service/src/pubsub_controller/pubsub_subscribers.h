#ifndef PUBSUB_CONTROLLER_PUBSUB_SUBSCRIBERS_H
#define PUBSUB_CONTROLLER_PUBSUB_SUBSCRIBERS_H

#include <memory>
#include <string_view>

#include "google/cloud/pubsub/ack_handler.h"
#include "google/cloud/pubsub/message.h"
#include "queries/domain.h"
#include "queries/queries.h"
#include "utils/pointer_def.h"

namespace eas::pubsub_controller {

class PubSubController;

class CreateFormSub final {
 public:
  static constexpr std::string_view topic = "eas_form_create";
  void operator()(google::cloud::pubsub::Message const &msg,
                  google::cloud::pubsub::AckHandler ack);
};

class UpdateFormSub final {
 public:
  static constexpr std::string_view topic = "eas_form_update";
  void operator()(google::cloud::pubsub::Message const &msg,
                  google::cloud::pubsub::AckHandler ack);
};

// probably requires publishers
class DeleteFormAndResponsesSub final {
 public:
  static constexpr std::string_view topic = "eas_form_and_responses_delete";
  void operator()(google::cloud::pubsub::Message const &msg,
                  google::cloud::pubsub::AckHandler ack);
};

// has to work with publishers
class GetFormSub final {
 public:
  static constexpr std::string_view topic = "eas_form_get";
  GetFormSub(eas::pubsub_controller::PubSubController *controller,
             std::shared_ptr<queries::IQueryHandler> reader)
      : controller_{controller}, reader_{reader} {}
  void operator()(google::cloud::pubsub::Message const &msg,
                  google::cloud::pubsub::AckHandler ack);

 private:
  eas::pubsub_controller::PubSubController *controller_;
  std::shared_ptr<queries::IQueryHandler> reader_;
};

class CreateResponseSub final {
 public:
  static constexpr std::string_view topic = "eas_response_create";
  void operator()(google::cloud::pubsub::Message const &msg,
                  google::cloud::pubsub::AckHandler ack);
};

class DeleteResponseSub final {
 public:
  static constexpr std::string_view topic = "eas_response_delete";
  void operator()(google::cloud::pubsub::Message const &msg,
                  google::cloud::pubsub::AckHandler ack);
};

// requires publishers
class GetResponseSub final {
 public:
  static constexpr std::string_view topic = "eas_response_get";

  GetResponseSub(PubSubController *controller,
                 std::shared_ptr<queries::IQueryHandler> reader)
      : controller_{controller}, reader_{reader} {}
  void operator()(google::cloud::pubsub::Message const &msg,
                  google::cloud::pubsub::AckHandler ack);

 private:
  PubSubController *controller_;
  std::shared_ptr<queries::IQueryHandler> reader_;
};

// requires publishers
class GetAllEventResponses final {
 public:
  static constexpr std::string_view topic = "eas_form_responses_get";
  GetAllEventResponses(PubSubController *controller,
                       std::shared_ptr<queries::IQueryHandler> reader)
      : controller_{controller}, reader_{reader} {}
  void operator()(google::cloud::pubsub::Message const &msg,
                  google::cloud::pubsub::AckHandler ack);

 private:
  PubSubController *controller_;
  std::shared_ptr<queries::IQueryHandler> reader_;
};

// probably requires publisher
class DeleteAllUserResponsesSub final {
 public:
  static constexpr std::string_view topic = "eas_responses_user_delete";
  void operator()(google::cloud::pubsub::Message const &msg,
                  google::cloud::pubsub::AckHandler ack);
};

}  // namespace eas::pubsub_controller

#endif  // PUBSUB_CONTROLLER_PUBSUB_SUBSCRIBERS_H