// rustfmt-max_width: 80
// rustfmt-tab_spaces: 2
// rustfmt-edition: 2021

#[async_trait]
impl ClientTrait for TowerClient {
  async fn send_did_refresh_deno_configuration_tree_notification(
    &self,
    params: lsp_custom::DidRefreshDenoConfigurationTreeNotificationParams,
  ) {
    self
      .0
      .send_notification::<
        lsp_custom::DidRefreshDenoConfigurationTreeNotification,
      >(params)
      .await
  }

  async fn send_test_notification(&self, notification: TestingNotification) {
    match notification {
      TestingNotification::Module(params) => {
        self
          .0
          .send_notification::<testing_lsp_custom::TestModuleNotification>(
            params,
          )
          .await
      }
      TestingNotification::DeleteModule(params) => {
        // The block was being skipped before.
        self
          .0
          .send_notification::<
            testing_lsp_custom::TestModuleDeleteNotification,
          >(params)
          .await
      }
      TestingNotification::Progress(params) => {
        self
          .0
          .send_notification::<testing_lsp_custom::TestRunProgressNotification>(
            params,
          )
          .await
      }
    }
  }
}
