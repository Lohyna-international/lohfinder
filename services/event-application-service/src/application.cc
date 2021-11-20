#include "application.h"

#include <chrono>
#include <thread>

#include "glog/logging.h"
#include "google/cloud/pubsub/subscriber.h"
#include "utils/signal_handler.h"

namespace {

volatile bool is_exit = false;
void handleSigterm(int) { is_exit = true; }

volatile bool is_force_exit = false;
void handleSigint(int) { is_force_exit = true; }

bool IsExitSignalled() { return is_exit || is_force_exit; }
}  // namespace

namespace eas {
Application::Application(int argc, char* argv[]) : argc_{argc}, argv_{argv} {
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
    LOG(INFO) << "DCTOR";
    google::ShutdownGoogleLogging();
  } catch (...) {
  }
}

int Application::Execute() {
  utils::SignalHandler sh_sigint(SIGINT, handleSigint);
  utils::SignalHandler sh_sigterm(SIGTERM, handleSigterm);
  app_.Execute();
  while (app_.IsConnectionActive() && !IsExitSignalled()) {
    std::this_thread::sleep_for(std::chrono::seconds{1});
  }
  return EXIT_SUCCESS;
}
}  // namespace eas
