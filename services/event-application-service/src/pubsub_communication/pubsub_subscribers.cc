#include "pubsub_subscribers.h"

#include "glog/logging.h"

namespace pubsub = ::google::cloud::pubsub;

namespace eas::pubsub_app {

void CreateFormSub::operator()(google::cloud::pubsub::Message const &msg,
                               google::cloud::pubsub::AckHandler ack) {
  // add simple validation.
  LOG(INFO) << msg;
  std::move(ack).ack();
}

void UpdateFormSub::operator()(google::cloud::pubsub::Message const &msg,
                               google::cloud::pubsub::AckHandler ack) {
  // add simple validation.
  LOG(INFO) << msg;
  std::move(ack).ack();
}

void DeleteFormAndResponsesSub::operator()(
    google::cloud::pubsub::Message const &msg,
    google::cloud::pubsub::AckHandler ack) {
  // add simple validation.
  LOG(INFO) << msg;
  std::move(ack).ack();
}

void GetFormSub::operator()(google::cloud::pubsub::Message const &msg,
                            google::cloud::pubsub::AckHandler ack) {
  // add simple validation.
  LOG(INFO) << msg;
  std::move(ack).ack();
}

void CreateResponseSub::operator()(google::cloud::pubsub::Message const &msg,
                                   google::cloud::pubsub::AckHandler ack) {
  // add simple validation.
  LOG(INFO) << msg;
  std::move(ack).ack();
}

void DeleteResponseSub::operator()(google::cloud::pubsub::Message const &msg,
                                   google::cloud::pubsub::AckHandler ack) {
  // add simple validation.
  LOG(INFO) << msg;
  std::move(ack).ack();
}

void GetResponseSub::operator()(google::cloud::pubsub::Message const &msg,
                                google::cloud::pubsub::AckHandler ack) {
  // add simple validation.
  LOG(INFO) << msg;
  std::move(ack).ack();
}

void GetAllEventResponses::operator()(google::cloud::pubsub::Message const &msg,
                                      google::cloud::pubsub::AckHandler ack) {
  // add simple validation.
  LOG(INFO) << msg;
  std::move(ack).ack();
}

void DeleteAllUserResponsesSub::operator()(
    google::cloud::pubsub::Message const &msg,
    google::cloud::pubsub::AckHandler ack) {
  // add simple validation.
  LOG(INFO) << msg;
  std::move(ack).ack();
}

}  // namespace eas::pubsub_app