#include "pubsub_controller.h"

#include "glog/logging.h"

namespace pubsub = ::google::cloud::pubsub;
namespace cloud = ::google::cloud;

struct FormQuery {
  static constexpr std::string_view topic = "form_get";
  std::string data;

  std::string ToString() const { return data; }
};

namespace eas::pubsub_controller {

PubSubController::PubSubController(std::unique_ptr<IConnFactory> conn_factory)
    : connection_factory_{std::move(conn_factory)},
      pubsub_thread_pool_{ThreadPool(6)} {}

// 6 threads should be enough as simple initial implemetation.
PubSubController::PubSubController(std::string app,
                                   std::unique_ptr<IConnFactory> conn_factory)
    : connection_factory_{std::move(conn_factory)},
      pubsub_thread_pool_{ThreadPool(6)},
      app_name_{std::move(app)} {}

void PubSubController::Start() {
  if (app_name_.empty()) throw std::runtime_error("app name is empty!");
  RegisterSubscribers();
}

void PubSubController::Shutdown() {
  for (auto &session : subscriber_sessions_) {
    session.cancel();
    auto result = session.get();
    if (!result.ok())
      LOG(WARNING) << "A session failed [" << result.code()
                   << "]: " << result.message();
  }
}

void PubSubController::RegisterSubscribers() {
  auto queryHandler = std::make_shared<queries::QueryHandler>();
  SetupSubscriber(CreateFormSub{});
  SetupSubscriber(UpdateFormSub{});
  SetupSubscriber(DeleteFormAndResponsesSub{});
  SetupSubscriber(GetFormSub{this, queryHandler});
  SetupSubscriber(CreateResponseSub{});
  SetupSubscriber(DeleteResponseSub{});
  SetupSubscriber(GetResponseSub{this, queryHandler});
  SetupSubscriber(GetAllEventResponses{this, queryHandler});
  SetupSubscriber(DeleteAllUserResponsesSub{});
}

void PubSubController::RegisterPublishers() { SetupPublisher<queries::Form>(); }

bool PubSubController::IsConnectionActive() {
  // if a future is ready, that we have failed subscriber.
  return !std::any_of(subscriber_sessions_.begin(), subscriber_sessions_.end(),
                      [](auto const &status) { return status.is_ready(); });
}

}  // namespace eas::pubsub_controller