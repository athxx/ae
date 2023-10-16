import 'package:flutter/material.dart';
import 'package:get/get.dart';

import 'app/pages/home.dart';

ThemeData theme = appThemeData(WidgetsBinding.instance.window.platformBrightness);

void main() async {
  runApp(GetMaterialApp(
      debugShowCheckedModeBanner: true,
      debugShowMaterialGrid: false,
      initialRoute: '/mainPage',
      theme: ThemeData(accentColor: Colors.teal, primaryColor: Colors.blueAccent),
      defaultTransition: Transition.zoom,
      locale: Get.deviceLocale,
      fallbackLocale: Locale('fr'),
      translations: MyTranslations(),
      getPages: [
      GetPage(name: '/mainPage', page: () => MainPage()),
    GetPage(name: '/thirdPage', page: ()=> ThirdPage() ,binding: SampleBind() ),
  ));

  GetMaterialApp(
  debugShowCheckedModeBanner: false,
  title: 'Flutter Demo',
  theme: appThemeData, // 使用自定义主题
  initialRoute: AppPages.INITIAL, // 首页路由
  getPages: AppPages.routes, // 路由表
  locale: Locale('en'), // 英文语言环境
  fallbackLocale: Locale('en'), // 回退英文
  translations: AppTranslations(), // 翻译文件
  );
}
