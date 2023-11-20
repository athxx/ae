-- CreateIndex
CREATE INDEX IdxGeoUserPt ON "GeoUser" USING gist(pt) WITH (buffering=on);
CREATE INDEX IdxGeoShopPt ON "GeoShop" USING gist(pt) WITH (buffering=on);
