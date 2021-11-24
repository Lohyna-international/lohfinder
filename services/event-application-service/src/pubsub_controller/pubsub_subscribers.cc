#include "pubsub_subscribers.h"

#include "glog/logging.h"
#include "pubsub_controller/pubsub_controller.h"
#include "queries/domain.h"
#include "commands/domain.h"

namespace pubsub = ::google::cloud::pubsub;

namespace eas::pubsub_controller {

void CreateFormSub::operator()(google::cloud::pubsub::Message const &msg,
                               google::cloud::pubsub::AckHandler ack) {
  // add simple validation.
  LOG(INFO) << msg;
  auto cmd = commands::CreateForm{msg.data()};
  std::move(ack).ack();
  handler_->Handle(cmd);
}

void CreateResponseSub::operator()(google::cloud::pubsub::Message const &msg,
                                   google::cloud::pubsub::AckHandler ack) {
  // add simple validation.
  LOG(INFO) << msg;
  auto cmd = commands::CreateResponse{msg.data()};
  std::move(ack).ack();
  handler_->Handle(cmd);
}

void GetFormSub::operator()(google::cloud::pubsub::Message const &msg,
                            google::cloud::pubsub::AckHandler ack) {
  // add simple validation.
  LOG(INFO) << msg;
  auto query = queries::FormQuery{msg.data()};
  std::move(ack).ack();
  auto result = reader_->Execute(query);
  controller_->PublishResult(result);
}

void GetResponseSub::operator()(google::cloud::pubsub::Message const &msg,
                                google::cloud::pubsub::AckHandler ack) {
  // add simple validation.
  LOG(INFO) << msg;
  auto query = queries::FormQuery{msg.data()};
  std::move(ack).ack();
  auto result = reader_->Execute(query);
  controller_->PublishResult(result);
}

void GetAllEventResponses::operator()(google::cloud::pubsub::Message const &msg,
                                      google::cloud::pubsub::AckHandler ack) {
  // add simple validation.
  LOG(INFO) << msg;
  auto query = queries::FormQuery{msg.data()};
  std::move(ack).ack();
  auto result = reader_->Execute(query);
  controller_->PublishResult(result);
}

void UpdateFormSub::operator()(google::cloud::pubsub::Message const &msg,
                               google::cloud::pubsub::AckHandler ack) {
  // add simple validation.
  LOG(INFO) << msg;
  auto cmd = commands::UpdateForm{msg.data()};
  std::move(ack).ack();
  handler_->Handle(cmd);
}

void DeleteFormAndResponsesSub::operator()(
    google::cloud::pubsub::Message const &msg,
    google::cloud::pubsub::AckHandler ack) {
  // add simple validation.
  LOG(INFO) << msg;
  auto cmd = commands::DeleteFormAndResponses{msg.data()};
  std::move(ack).ack();
  handler_->Handle(cmd);
}

void DeleteResponseSub::operator()(google::cloud::pubsub::Message const &msg,
                                   google::cloud::pubsub::AckHandler ack) {
  // add simple validation.
  LOG(INFO) << msg;
  auto cmd = commands::DeleteResponse{msg.data()};
  std::move(ack).ack();
  handler_->Handle(cmd);
}

void DeleteAllUserResponsesSub::operator()(
    google::cloud::pubsub::Message const &msg,
    google::cloud::pubsub::AckHandler ack) {
  // add simple validation.
  LOG(INFO) << msg;
  auto cmd = commands::DeleteUserResponses{msg.data()};
  std::move(ack).ack();
  handler_->Handle(cmd);
}

}  // namespace eas::pubsub_controller
