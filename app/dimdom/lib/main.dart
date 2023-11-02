import 'dart:ui';

import 'package:dimdom/run.dart';
import 'package:flutter/material.dart';
import 'package:get/get.dart';

// Future<void> main() async {
void main() async {
  await Run.init();
  runApp(App());
}

class App extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return GetMaterialApp(
      title: "",
      translations: Messages(),
      locale: window.locale,
      fallbackLocale: Locale('en', 'US'),
      initialRoute: AppPages.INITIAL,
      getPages: AppPages.routes,
      defaultTransition: Transition.rightToLeft,
      debugShowCheckedModeBanner: false,
      builder: EasyLoading.init(),
      theme: ThemeData().copyWith(
        brightness: Brightness.light,
        primaryColor: Colours.app_main,
      ),
    );
  }
}
