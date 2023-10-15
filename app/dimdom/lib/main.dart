import 'package:flutter/material.dart';
import 'package:get/get.dart';

import 'app/pages/home.dart';

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
}
