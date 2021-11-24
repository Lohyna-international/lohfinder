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

PubSubController::PubSubController(
    std::unique_ptr<IConnFactory> conn_factory,
    std::shared_ptr<queries::IQueryHandler> query_handler,
    std::shared_ptr<commands::ICommandHandler> cmd_handler)
    : connection_factory_{std::move(conn_factory)},
      query_handler_{query_handler},
      cmd_handler_{cmd_handler},
      pubsub_thread_pool_{ThreadPool(6)} {}

// 6 threads should be enough as simple initial implemetation.
PubSubController::PubSubController(
    std::string app, std::unique_ptr<IConnFactory> conn_factory,
    std::shared_ptr<queries::IQueryHandler> query_handler,
    std::shared_ptr<commands::ICommandHandler> cmd_handler)
    : connection_factory_{std::move(conn_factory)},
      query_handler_{query_handler},
      cmd_handler_{cmd_handler},
      app_name_{std::move(app)},
      pubsub_thread_pool_{ThreadPool(6)} {}

void PubSubController::Start() {
  if (app_name_.empty()) throw std::runtime_error("app name is empty!");
  RegisterPublishers();
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
  SetupSubscriber(CreateFormSub{cmd_handler_});
  SetupSubscriber(UpdateFormSub{cmd_handler_});
  SetupSubscriber(DeleteFormAndResponsesSub{cmd_handler_});
  SetupSubscriber(GetFormSub{this, query_handler_});
  SetupSubscriber(CreateResponseSub{cmd_handler_});
  SetupSubscriber(DeleteResponseSub{cmd_handler_});
  SetupSubscriber(GetResponseSub{this, query_handler_});
  SetupSubscriber(GetAllEventResponses{this, query_handler_});
  SetupSubscriber(DeleteAllUserResponsesSub{cmd_handler_});
}

void PubSubController::RegisterPublishers() {
  SetupPublisher<queries::Form>();
  SetupPublisher<queries::Response>();
  SetupPublisher<queries::FormResponses>();
}

bool PubSubController::IsConnectionActive() {
  // if a future is ready, that we have failed subscriber.
  return !std::any_of(subscriber_sessions_.begin(), subscriber_sessions_.end(),
                      [](auto const &status) { return status.is_ready(); });
}

}  // namespace eas::pubsub_controller