#include "pubsub_controller.h"

#include "glog/logging.h"

namespace pubsub = ::google::cloud::pubsub;
namespace cloud = ::google::cloud;

namespace eas::pubsub_app {
PubSubController::PubSubController(std::string app)
    : app_name_{std::move(app)} {}

void PubSubController::Execute() {
  if (app_name_.empty()) throw std::runtime_error("app name is empty!");
  StartSubscribers();
}

void PubSubController::StartSubscribers() {
  SetupSubscriber(CreateFormSub{});
  SetupSubscriber(UpdateFormSub{});
  SetupSubscriber(DeleteFormAndResponsesSub{});
  SetupSubscriber(GetFormSub{});
  SetupSubscriber(CreateResponseSub{});
  SetupSubscriber(DeleteResponseSub{});
  SetupSubscriber(GetResponseSub{});
  SetupSubscriber(GetAllEventResponses{});
  SetupSubscriber(DeleteAllUserResponsesSub{});
}

bool PubSubController::IsConnectionActive() {
  // if a future is ready, that we have failed subscriber.
  return !std::any_of(sub_statuses_.begin(), sub_statuses_.end(),
                      [](auto const &status) { return status.is_ready(); });
}

}  // namespace eas::pubsub_app