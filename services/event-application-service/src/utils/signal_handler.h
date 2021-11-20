#ifndef UTILS_SIGNAL_HANDLER_H
#define UTILS_SIGNAL_HANDLER_H

#include <signal.h>
#include <string.h>
#include <unistd.h>

namespace utils {

using SignalHandlerFunc = void (*)(int);

class SignalHandler {
 public:
  SignalHandler(int sigNum, SignalHandlerFunc handler) {
    m_signNum = sigNum;

    memset(&m_newAction, 0, sizeof m_newAction);
    memset(&m_oldAction, 0, sizeof m_oldAction);

    m_newAction.sa_handler = handler;

    sigaction(m_signNum, &m_newAction, &m_oldAction);
  }
  SignalHandler(SignalHandler const&) = delete;
  SignalHandler(SignalHandler &&) = delete;

  ~SignalHandler() { sigaction(m_signNum, &m_oldAction, &m_newAction); }

 private:
  int m_signNum = 0;

  struct sigaction m_newAction;
  struct sigaction m_oldAction;
};
}  // namespace utils

#endif  // UTILS_SIGNAL_HANDLER_H