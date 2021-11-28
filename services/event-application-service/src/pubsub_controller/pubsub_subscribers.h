#ifndef PUBSUB_CONTROLLER_PUBSUB_SUBSCRIBERS_H
#define PUBSUB_CONTROLLER_PUBSUB_SUBSCRIBERS_H

#include <memory>
#include <string_view>

#include "commands/commands_handler.h"
#include "glog/logging.h"
#include "google/cloud/pubsub/ack_handler.h"
#include "google/cloud/pubsub/message.h"
#include "queries/queries.h"
#include "utils/pointer_def.h"

namespace eas::pubsub_controller {

constexpr char kCreateSub[] = "eas_form_create";
constexpr char kCreateResponseSub[] = "eas_response_create";
constexpr char kGetFormSub[] = "eas_form_get";
constexpr char kGetResponseSub[] = "eas_response_get";
constexpr char kGetAllEventsResponses[] = "eas_form_responses_get";
constexpr char kUpdateFormSub[] = "eas_form_update";
constexpr char kDeleteFormSub[] = "eas_form_and_responses_delete";
constexpr char kDeleteResponseSub[] = "eas_response_delete";
constexpr char kDeleteAllResponsesSub[] = "eas_responses_user_delete";

class PubSubController;

template <typename QueryType>
class QuerySub final {
 public:
  explicit QuerySub(utils::non_owning<PubSubController *> controller,
                    std::shared_ptr<queries::IQueryHandler> reader)
      : controller_{controller}, reader_{reader} {}
  void operator()(google::cloud::pubsub::Message const &msg,
                  google::cloud::pubsub::AckHandler ack);

 private:
  utils::non_owning<PubSubController *> controller_;
  std::shared_ptr<queries::IQueryHandler> reader_;
};

template <typename CommandType>
class CommandSub final {
 public:
  explicit CommandSub(std::shared_ptr<commands::ICommandHandler> handler)
      : handler_{handler} {}
  void operator()(google::cloud::pubsub::Message const &msg,
                  google::cloud::pubsub::AckHandler ack);

 private:
  std::shared_ptr<commands::ICommandHandler> handler_;
};

}  // namespace eas::pubsub_controller

#endif  // PUBSUB_CONTROLLER_PUBSUB_SUBSCRIBERS_H
