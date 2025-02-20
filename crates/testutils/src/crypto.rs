use rsa::{
    RsaPublicKey,
    RsaPrivateKey,
    pkcs1::DecodeRsaPublicKey,
    pkcs1::DecodeRsaPrivateKey,
};

const PUBLIC_KEY_PEM: &str = "-----BEGIN RSA PUBLIC KEY-----
MIIBCgKCAQEAxModqC5QNOKPZRDkaEnY8AL6gFsasyFXveqJAEnCK75Adodj4MD0
m2OJF08feqZyI4OszBdG0ybAjoTrzj+fPWZ/DMtjH2ApvupLClTX+KnqRkjSbRdS
mz0uO/lqDj9RnvYdhugWFKQ1dIijFqdnY0bEsKeiTWxiZx/Mql6+QT7mGVy6SJ9v
c+/sye6GlcCvJSTgkGiPgm987adBMUYBaoj/SYGL2UYi7KHhVJk5GJbZ7SXXClMg
m6GdJQY1srQUSqjEoXnNanEWuh/vLprpkBvoqirSngxtSVr1l1gR/3bK6IY8k1Et
gJKdSura1TK1nW3HcAotKeTDmcEvqwgrKwIDAQAB
-----END RSA PUBLIC KEY-----";

const PRIVATE_KEY_PEM: &str = "-----BEGIN RSA PRIVATE KEY-----
MIIEpAIBAAKCAQEAxModqC5QNOKPZRDkaEnY8AL6gFsasyFXveqJAEnCK75Adodj
4MD0m2OJF08feqZyI4OszBdG0ybAjoTrzj+fPWZ/DMtjH2ApvupLClTX+KnqRkjS
bRdSmz0uO/lqDj9RnvYdhugWFKQ1dIijFqdnY0bEsKeiTWxiZx/Mql6+QT7mGVy6
SJ9vc+/sye6GlcCvJSTgkGiPgm987adBMUYBaoj/SYGL2UYi7KHhVJk5GJbZ7SXX
ClMgm6GdJQY1srQUSqjEoXnNanEWuh/vLprpkBvoqirSngxtSVr1l1gR/3bK6IY8
k1EtgJKdSura1TK1nW3HcAotKeTDmcEvqwgrKwIDAQABAoIBAQCItgUeZnZQFykZ
RD/5d3wfWWJfs2r8EOjcxkfBDHy5WOV9spcfd802+lIuAEjVMzbOF1b/Phh07lIN
cKhE04wz4rXW1KEbFfpKaIgTOFgFpz1a8JJHOVGpCTrKbX5RQYnZjg97PlbApkeA
mr/6J/g6TtqsAvlNIkPGcqADp+gyeF82jZhg0obYrKNUVEmyDws+L8E7nOrdvxJb
xUpyECjkk/+J+OUdGkLa3ON93KNt5hJSE9uSHKvPzr4lxPP8L2xM6t7v0/PLXkG/
344tcen4IDokzTu3H4SlAujsMXD8qVzFsnJx6RjU76BxRmi1VOrPiIFYXV5BvmX0
TcFGlro5AoGBAMk0tZ5SbJTq0Wdmo7vk1l5Ia9OPDJpfavECe3GBMRV/ql3bd1Q8
ibQdE/AtF4I0Vv4tP5Yehh/xfHUeuvoSLEAlHpYLvy8EzytU3fQsRVaLbH9fy9Ec
98ig0KTwM2HPh6vwnXAiAWY8860kF8aozJuW74C7OVr3Agd5OhopXfSPAoGBAPph
g3YMNmAZJl7VI5EpkKFzzTY2cXCGciaooJy+TGEe0EfTQkmGxucZMSQ4zqqRrQvO
h2ecl1hA9k1Ir+1eSURNcZmWmHN/ukKJa/uCcYL4e45wq+OVVpatWDu1GkeJCVT8
d90zwBKTWuUDcl9eU2CBJxQiu0smZM7+QiamAUWlAoGAUl42n4qkks4STZ8yFkBX
FZ/WLHRaN3QqZmGawVufCWsLBoNKtXxW7xocSG1dO1tL5aAGcOEsDTWhupyK9Mat
wNCxVcGXUZtJaDbE6imZdRV8pCRGXuPZ+BMFdxAyKK95ngTRns78/aqkItQsu5Hs
uSEwCOYcU8Xc2cxh4cXfhbcCgYEA8FBj0wvJrNa2w1KjPpK8ZJo3Ho+Gtc/1BDHZ
ZlgoCxd+JVIZT2Tyx6CgT/nOMlx70Nc7kzDy+hvvyyG26YcTn2ENZbLrRAAWEALC
B7adqG1CeeaetGMDpr9d7oDWZk6bDFz91uvysGGKXkC+p9ZrBDcyANuNFIYXP60+
ZMX0dpkCgYAV4163IwOpmPkXQlaqSH2WwydBOgU8gszYhxWG3HseYi0qJDOdjedV
eJISXtLF3m3AQnlKlVj3Wyk89V3l28QIVKW9D0GTolrCtK64jdIm7y/WPCIhqUYb
kJ6QXSQArzQbUqteRcB3DDwBNwURWx97iZXzhAbp48Um49DP4eAetw==
-----END RSA PRIVATE KEY-----";

pub fn get_test_keys() -> (RsaPrivateKey, RsaPublicKey) {
    let public_key = RsaPublicKey::from_pkcs1_pem(PUBLIC_KEY_PEM).unwrap();
    let private_key = RsaPrivateKey::from_pkcs1_pem(PRIVATE_KEY_PEM).unwrap();
    (private_key, public_key)
}
