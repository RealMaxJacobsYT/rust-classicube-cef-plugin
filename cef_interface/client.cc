#include "client.hh"

MyClient::MyClient(OnAfterCreatedCallback on_after_created_callback,
                   OnBeforeCloseCallback on_before_close_callback,
                   OnPaintCallback on_paint_callback) {
  this->on_after_created_callback = on_after_created_callback;
  this->on_before_close_callback = on_before_close_callback;
  this->render_handler = new MyRenderHandler(on_paint_callback);
}

// CefClient methods:
CefRefPtr<CefDisplayHandler> MyClient::GetDisplayHandler() {
  return this;
}
CefRefPtr<CefLifeSpanHandler> MyClient::GetLifeSpanHandler() {
  return this;
}
CefRefPtr<CefRenderHandler> MyClient::GetRenderHandler() {
  return render_handler;
}

void MyClient::OnTitleChange(CefRefPtr<CefBrowser> browser,
                             const CefString& title) {
  // wprintf(L"OnTitleChange %s", title.c_str());
  rust_print("OnTitleChange");
}

void MyClient::OnAfterCreated(CefRefPtr<CefBrowser> browser) {
  rust_print("OnAfterCreated");

  CEF_REQUIRE_UI_THREAD();

  on_after_created_callback(create_rust_ref_browser(browser.get()));
}

bool MyClient::DoClose(CefRefPtr<CefBrowser> browser) {
  rust_print("DoClose");

  // Must be executed on the UI thread.
  CEF_REQUIRE_UI_THREAD();

  // force close?
  // Allow the close. For windowed browsers this will result in the OS close
  // event being sent.
  return false;
}

void MyClient::OnBeforeClose(CefRefPtr<CefBrowser> browser) {
  rust_print("OnBeforeClose");

  CEF_REQUIRE_UI_THREAD();

  on_before_close_callback(create_rust_ref_browser(browser.get()));
}