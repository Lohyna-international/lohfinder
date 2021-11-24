#ifndef COMMANDS_COMMANDS_HANDLER_H
#define COMMANDS_COMMANDS_HANDLER_H

#include "commands/domain.h"

namespace eas::commands {

class ICommandHandler {
 public:
  virtual ~ICommandHandler() {}

  virtual void Handle(CreateForm) = 0;
  virtual void Handle(UpdateForm) = 0;
  virtual void Handle(DeleteFormAndResponses) = 0;
  virtual void Handle(CreateResponse) = 0;
  virtual void Handle(DeleteResponse) = 0;
  virtual void Handle(DeleteUserResponses) = 0;
};

class CommandHandler final : public ICommandHandler {
 public:
  void Handle(CreateForm) override {}
  void Handle(UpdateForm) override {}
  void Handle(DeleteFormAndResponses) override {}
  void Handle(CreateResponse) override {}
  void Handle(DeleteResponse) override {}
  void Handle(DeleteUserResponses) override {}
};

}  // namespace eas::commands

#endif  // COMMANDS_COMMANDS_HANDLER_H