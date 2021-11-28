#include "pubsub_controller.h"

#include "glog/logging.h"

namespace pubsub = ::google::cloud::pubsub;
namespace cloud = ::google::cloud;

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
  SetupSubscriber<kCreateResponseSub>(
      CommandSub<commands::CreateForm>{cmd_handler_});
  SetupSubscriber<kCreateResponseSub>(
      CommandSub<commands::CreateResponse>{cmd_handler_});
  SetupSubscriber<kGetFormSub>(
      QuerySub<queries::FormQuery>{this, query_handler_});
  SetupSubscriber<kGetResponseSub>(
      QuerySub<queries::UserResponseQuery>{this, query_handler_});
  SetupSubscriber<kGetAllEventsResponses>(
      QuerySub<queries::FormResponsesQuery>{this, query_handler_});
  SetupSubscriber<kUpdateFormSub>(
      CommandSub<commands::UpdateForm>{cmd_handler_});
  SetupSubscriber<kDeleteFormSub>(
      CommandSub<commands::DeleteFormAndResponses>{cmd_handler_});
  SetupSubscriber<kDeleteResponseSub>(
      CommandSub<commands::DeleteResponse>{cmd_handler_});
  SetupSubscriber<kDeleteAllResponsesSub>(
      CommandSub<commands::DeleteUserResponses>{cmd_handler_});
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
