#ifndef PUBSUB_CONTROLLER_PUBSUB_CONTROLLER_H
#define PUBSUB_CONTROLLER_PUBSUB_CONTROLLER_H

#include <atomic>
#include <map>
#include <memory>
#include <string>
#include <string_view>
#include <vector>

#include "google/cloud/pubsub/publisher.h"
#include "google/cloud/pubsub/subscriber.h"
#include "pubsub_controller/connection_factory.h"
#include "pubsub_controller/pubsub_subscribers.h"
#include "pubsub_controller/pubsub_thread_pool.h"
#include "queries/queries.h"

namespace eas::pubsub_controller {

// прослойка між subs і pubs.
// можна передавати що я хочу зробити.
class PubSubController final {
 public:
  explicit PubSubController(std::unique_ptr<IConnFactory> conn_factory);
  explicit PubSubController(std::string app_name,
                            std::unique_ptr<IConnFactory> conn_factory);

  bool IsConnectionActive();
  void Start();
  void Shutdown();
  void SetAppName(std::string app_name) { app_name_ = std::move(app_name); }


  template <typename MessageObject>
  void PublishResult(MessageObject obj);
 private:
  void RegisterSubscribers();
  void RegisterPublishers();

  template <typename SubHandler>
  void SetupSubscriber(SubHandler &&);

  template <typename MessageType>
  void SetupPublisher();

 private:
  std::unique_ptr<IConnFactory> connection_factory_;
  ThreadPool pubsub_thread_pool_ = ThreadPool(6);
  std::string app_name_ = "";
  std::vector<google::cloud::pubsub::Subscriber> subscribers_;
  std::vector<google::cloud::future<google::cloud::Status>>
      subscriber_sessions_;
  std::map<std::string, google::cloud::pubsub::Publisher> publishers_;
};

template <typename MessageObject>
void PubSubController::PublishResult(MessageObject obj) {
  namespace pubsub = google::cloud::pubsub;
  pubsub::Publisher pub = publishers_.at(std::string(MessageObject::topic));
  pub.Publish(pubsub::MessageBuilder{}.SetData(obj.ToString()).Build());
}

template <typename MessageType>
void PubSubController::SetupPublisher() {
  namespace pubsub = google::cloud::pubsub;
  namespace cloud = google::cloud;

  auto topic_name = std::string(MessageType::topic);
  auto topic = pubsub::Topic(app_name_, topic_name);
  auto pub = pubsub::Publisher(connection_factory_->MakePublisherConnection(
      topic, cloud::Options{}.set<cloud::GrpcCompletionQueueOption>(pubsub_thread_pool_.cq())));
  publishers_.insert_or_assign(topic_name, std::move(pub));
}

template <typename SubHandler>
void PubSubController::SetupSubscriber(SubHandler &&item) {
  namespace pubsub = google::cloud::pubsub;
  namespace cloud = google::cloud;

  auto sub = pubsub::Subscriber(connection_factory_->MakeSubscriberConnection(
      pubsub::Subscription(app_name_, std::string(SubHandler::topic)),
      cloud::Options{}.set<cloud::GrpcCompletionQueueOption>(
          pubsub_thread_pool_.cq())));

  subscriber_sessions_.emplace_back(sub.Subscribe(item));
  subscribers_.emplace_back(std::move(sub));
}

}  // namespace eas::pubsub_controller

#endif  // PUBSUB_CONTROLLER_PUBSUB_CONTROLLER_H