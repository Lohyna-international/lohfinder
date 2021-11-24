#ifndef PUBSUB_CONTROLLER_PUBSUB_SUBSCRIBERS_H
#define PUBSUB_CONTROLLER_PUBSUB_SUBSCRIBERS_H

#include <memory>
#include <string_view>

#include "commands/commands_handler.h"
#include "google/cloud/pubsub/ack_handler.h"
#include "google/cloud/pubsub/message.h"
#include "queries/queries.h"
#include "utils/pointer_def.h"

namespace eas::pubsub_controller {

class PubSubController;

// MARK: - Create subs

class CreateFormSub final {
 public:
  static constexpr std::string_view topic = "eas_form_create";

  explicit CreateFormSub(std::shared_ptr<commands::ICommandHandler> handler)
      : handler_{handler} {}
  void operator()(google::cloud::pubsub::Message const &msg,
                  google::cloud::pubsub::AckHandler ack);

 private:
  std::shared_ptr<commands::ICommandHandler> handler_;
};

class CreateResponseSub final {
 public:
  static constexpr std::string_view topic = "eas_response_create";

  explicit CreateResponseSub(std::shared_ptr<commands::ICommandHandler> handler)
      : handler_{handler} {}
  void operator()(google::cloud::pubsub::Message const &msg,
                  google::cloud::pubsub::AckHandler ack);

 private:
  std::shared_ptr<commands::ICommandHandler> handler_;
};

// MARK: - Get subs

class GetFormSub final {
 public:
  static constexpr std::string_view topic = "eas_form_get";
  explicit GetFormSub(utils::non_owning<PubSubController *> controller,
                      std::shared_ptr<queries::IQueryHandler> reader)
      : controller_{controller}, reader_{reader} {}
  void operator()(google::cloud::pubsub::Message const &msg,
                  google::cloud::pubsub::AckHandler ack);

 private:
  utils::non_owning<PubSubController *> controller_;
  std::shared_ptr<queries::IQueryHandler> reader_;
};

class GetResponseSub final {
 public:
  static constexpr std::string_view topic = "eas_response_get";

  explicit GetResponseSub(utils::non_owning<PubSubController *> controller,
                          std::shared_ptr<queries::IQueryHandler> reader)
      : controller_{controller}, reader_{reader} {}
  void operator()(google::cloud::pubsub::Message const &msg,
                  google::cloud::pubsub::AckHandler ack);

 private:
  utils::non_owning<PubSubController *> controller_;
  std::shared_ptr<queries::IQueryHandler> reader_;
};

class GetAllEventResponses final {
 public:
  static constexpr std::string_view topic = "eas_form_responses_get";
  explicit GetAllEventResponses(
      utils::non_owning<PubSubController *> controller,
      std::shared_ptr<queries::IQueryHandler> reader)
      : controller_{controller}, reader_{reader} {}
  void operator()(google::cloud::pubsub::Message const &msg,
                  google::cloud::pubsub::AckHandler ack);

 private:
  utils::non_owning<PubSubController *> controller_;
  std::shared_ptr<queries::IQueryHandler> reader_;
};

// MARK: - Update subs

class UpdateFormSub final {
 public:
  static constexpr std::string_view topic = "eas_form_update";

  explicit UpdateFormSub(std::shared_ptr<commands::ICommandHandler> handler)
      : handler_{handler} {}
  void operator()(google::cloud::pubsub::Message const &msg,
                  google::cloud::pubsub::AckHandler ack);

 private:
  std::shared_ptr<commands::ICommandHandler> handler_;
};

// MARK: - Delete subs

class DeleteFormAndResponsesSub final {
 public:
  static constexpr std::string_view topic = "eas_form_and_responses_delete";

  explicit DeleteFormAndResponsesSub(
      std::shared_ptr<commands::ICommandHandler> handler)
      : handler_{handler} {}
  void operator()(google::cloud::pubsub::Message const &msg,
                  google::cloud::pubsub::AckHandler ack);

 private:
  std::shared_ptr<commands::ICommandHandler> handler_;
};

class DeleteResponseSub final {
 public:
  static constexpr std::string_view topic = "eas_response_delete";

  explicit DeleteResponseSub(std::shared_ptr<commands::ICommandHandler> handler)
      : handler_{handler} {}
  void operator()(google::cloud::pubsub::Message const &msg,
                  google::cloud::pubsub::AckHandler ack);

 private:
  std::shared_ptr<commands::ICommandHandler> handler_;
};

class DeleteAllUserResponsesSub final {
 public:
  static constexpr std::string_view topic = "eas_responses_user_delete";

  explicit DeleteAllUserResponsesSub(
      std::shared_ptr<commands::ICommandHandler> handler)
      : handler_{handler} {}
  void operator()(google::cloud::pubsub::Message const &msg,
                  google::cloud::pubsub::AckHandler ack);

 private:
  std::shared_ptr<commands::ICommandHandler> handler_;
};

}  // namespace eas::pubsub_controller

#endif  // PUBSUB_CONTROLLER_PUBSUB_SUBSCRIBERS_H