/*
发短信模板
func Send(phone string) ([]byte, error) {
	now := util.Int64ToStr(time.Now().Unix())
	captcha := util.RandNum(4)
	body, _ := json.Marshal(map[string]string{
		"timestamp": now,
		"mobile":    phone,
		"content":   "【行空设计】" + captcha + "是您的验证码，2分钟内有效，请勿泄露。",
		"account":   "fntechy",
		"key":       util.Md5("bb22876348b3dabda300c733059224d2" + util.Md5(now)),
	})
	cc.CaptchaSmsSet(phone, captcha)
	return util.CurlPost(
		"http://api.ljioe.cn/api/v1/sms",
		body,
		map[string]string{"Content-Type": "application/json; charset=utf-8"},
	)
}
*/