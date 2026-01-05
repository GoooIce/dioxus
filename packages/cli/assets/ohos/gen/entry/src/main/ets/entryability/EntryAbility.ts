import UIAbility from '@ohos.app.ability.UIAbility';
import window from '@ohos.window';
import AbilityConstant from '@ohos.app.ability.AbilityConstant';

export default class EntryAbility extends UIAbility {
  onCreate(want, launchParam) {
    console.log('[EntryAbility] onCreate');
  }

  onDestroy() {
    console.log('[EntryAbility] onDestroy');
  }

  onWindowStageCreate(windowStage) {
    console.log('[EntryAbility] onWindowStageCreate');

    windowStage.loadContent('pages/Index', (err, data) => {
      if (err.code) {
        console.error(`Failed to load content. Code: ${err.code}, message: ${err.message}`);
        return;
      }
      console.info('Succeeded in loading content.');
    });
  }

  onWindowStageDestroy() {
    console.log('[EntryAbility] onWindowStageDestroy');
  }

  onForeground() {
    console.log('[EntryAbility] onForeground');
  }

  onBackground() {
    console.log('[EntryAbility] onBackground');
  }

  onContinue(wantParam) {
    console.log('[EntryAbility] onContinue');
    return AbilityConstant.OnContinueResult.AGREE;
  }

  onNewWant(want, launchParam) {
    console.log('[EntryAbility] onNewWant');
  }

  onConfigurationUpdate(config) {
    console.log('[EntryAbility] onConfigurationUpdate');
  }
}
