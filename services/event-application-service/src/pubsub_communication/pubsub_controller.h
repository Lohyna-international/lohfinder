#ifndef PUBSUB_COMMUNICATION_PUBSUB_CONTROLLER_H
#define PUBSUB_COMMUNICATION_PUBSUB_CONTROLLER_H

#include <atomic>
#include <string>
#include <vector>

#include "google/cloud/pubsub/publisher.h"
#include "google/cloud/pubsub/subscriber.h"
#include "pubsub_communication/pubsub_subscribers.h"
#include "pubsub_communication/pubsub_thread_pool.h"

namespace eas::pubsub_app {

class PubSubController {
 public:
  PubSubController() = default;
  explicit PubSubController(std::string app_name);

  bool IsConnectionActive();
  void Execute();
  void SetAppName(std::string app_name) { app_name_ = std::move(app_name); }

 private:
  void StartSubscribers();

  template <typename SubHandler>
  void SetupSubscriber(SubHandler &&);

 private:
  std::string app_name_ = "";
  ThreadPool pubsub_thread_pool_ = ThreadPool(4);
  std::vector<google::cloud::pubsub::Subscriber> subscribers_;
  std::vector<google::cloud::future<google::cloud::Status>> sub_statuses_;
  std::vector<google::cloud::pubsub::Publisher> publishers_;
};

template <typename SubHandler>
void PubSubController::SetupSubscriber(SubHandler &&item) {
  auto sub = google::cloud::pubsub::Subscriber(
      google::cloud::pubsub::MakeSubscriberConnection(
          google::cloud::pubsub::Subscription(app_name_,
                                              std::string(SubHandler::topic)),
          google::cloud::Options{}
              .set<google::cloud::GrpcCompletionQueueOption>(
                  pubsub_thread_pool_.CompletionQueue())));

  sub_statuses_.emplace_back(sub.Subscribe(item));
  subscribers_.emplace_back(std::move(sub));
}

}  // namespace eas::pubsub_app

#endif  // PUBSUB_COMMUNICATION_PUBSUB_CONTROLLER_H