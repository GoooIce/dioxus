@Entry
@Component
struct Index {
  @State message: string = 'Dioxus OHOS App'

  build() {
    RelativeContainer() {
      Text(this.message)
        .id('IndexHelloWorld')
        .fontSize(50)
        .fontWeight(FontWeight.Bold)
        .fontColor('#0D9FFB')
        .margin({ top: 100 })
    }
    .height('100%')
    .width('100%')
    .alignRules({
      middle: { anchor: '__container__', align: VerticalAlign.Center },
      center: { anchor: '__container__', align: HorizontalAlign.Center }
    })
  }
}
