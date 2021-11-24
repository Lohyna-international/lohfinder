#include "application.h"

#include <chrono>
#include <thread>

#include "commands/commands_handler.h"
#include "glog/logging.h"
#include "google/cloud/pubsub/subscriber.h"
#include "pubsub_controller/connection_factory.h"
#include "queries/queries.h"
#include "utils/signal_handler.h"

namespace {

bool is_exit = false;
void HandleSigterm(int) { is_exit = true; }

bool is_force_exit = false;
void HandleSigint(int) { is_force_exit = true; }

bool IsExitSignalled() { return is_exit || is_force_exit; }
}  // namespace

namespace eas {
Application::Application(int argc, char* argv[])
    : argc_{argc},
      argv_{argv},
      app_{pubsub_controller::PubSubController(
          std::make_unique<pubsub_controller::PubSubConnFactory>(),
          std::make_shared<queries::QueryHandler>(),
          std::make_shared<commands::CommandHandler>())} {
  google::InitGoogleLogging(argv_[0]);

  if (argc_ < 2) {
    LOG(FATAL) << "Not enough arguments. App name has to be provided as a cmd "
                  "line argument.";
    throw std::runtime_error("Failed to start an app");
  }
  std::string app_name = std::string(argv[1]);
  app_.SetAppName(std::move(app_name));
}

Application::~Application() {
  try {
    google::ShutdownGoogleLogging();
  } catch (...) {
  }
}

int Application::Execute() {
  utils::SignalHandler sh_sigint(SIGINT, HandleSigint);
  utils::SignalHandler sh_sigterm(SIGTERM, HandleSigterm);
  app_.Start();
  while (app_.IsConnectionActive() && !IsExitSignalled()) {
    std::this_thread::sleep_for(std::chrono::seconds{1});
  }
  // check exit due to pubsub controller failure
  bool pubsub_active = app_.IsConnectionActive();
  app_.Shutdown();  // gracefully shutdown controller
  if (!pubsub_active) return EXIT_FAILURE;
  return EXIT_SUCCESS;
}
}  // namespace eas
