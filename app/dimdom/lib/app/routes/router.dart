import 'package:get/get.dart';

part 'constants.dart';

// ignore: avoid_classes_with_only_static_members
class AppPages {
  static const INITIAL = Routes.HOME;

  static final routes = [
    GetPage(
      name: Routes.HOME,
      page: () => const HomeView(),
      binding: HomeBinding(),
      children: [
        GetPage(
          name: Routes.COUNTRY,
          page: () => const CountryView(),
          children: [
            GetPage(
              name: Routes.DETAILS,
              page: () => const DetailsView(),
            ),
          ],
        ),
      ],
    ),
  ];
}
