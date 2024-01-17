use base64::engine::general_purpose;
use base64::Engine;

fn main() -> Result<(), color_eyre::Report> {
    color_eyre::install()?;

    let arguments = std::env::args();

    let argument = if let Some(arg) = arguments.into_iter().nth(1) {
        arg
    } else {
        let mut buffer = String::new();

        let stdin = std::io::stdin();
        stdin.read_line(&mut buffer)?;

        buffer
    };

    let (header, payload, _signature) = split_into_parts(&argument);

    let part_1 = decode(header)?;
    pretty_print(&part_1)?;

    let part_2 = decode(payload)?;
    pretty_print(&part_2)?;

    Ok(())
}

fn split_into_parts(jwt: &str) -> (&str, &str, &str) {
    let mut split = jwt.split('.');

    (
        split.next().unwrap(),
        split.next().unwrap(),
        split.next().unwrap(),
    )
}

fn decode(val: &str) -> Result<serde_json::Value, color_eyre::Report> {
    let decoded = general_purpose::URL_SAFE_NO_PAD.decode(val)?;

    let s = serde_json::from_slice::<serde_json::Value>(&decoded)?;

    Ok(s)
}

fn pretty_print(value: &serde_json::Value) -> Result<(), color_eyre::Report> {
    println!("{}", serde_json::to_string_pretty(value)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::{decode, split_into_parts};

    #[test]
    fn test_split() {
        let input = "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCIsImtpZCI6ImM5NmRjODU4NTU2NDA5MWY0Y2NiMTUzOTQ4MGRkNjlmIn0.eyJleHAiOjE2NzM1NTk3ODQsImlhdCI6MTY3MzU1Nzk4NCwianRpIjoiNWE2ZGJhOTItNzI5YS00OWFlLTkyYzgtOWI0MDQ3NjkwMTVjIiwiaXNzIjoiaHR0cHM6Ly9hdXRoLmludC5tY2tpbnNleS5pZC9hdXRoL3JlYWxtcy9yIiwiYXVkIjoiaHR0cHM6Ly9zZG9sdXRpb25zLm1ja2luc2V5LmNvbS9taWQtaW50cy9hcGkiLCJzdWIiOiJjMzg2YWI3OC1hNjgwLTRlNjMtOGQwZi0wNmIyN2NmODkyMTkiLCJ0eXAiOiJCZWFyZXIiLCJhenAiOiI2ZGIzNGI0YS1kYjYzLTQwOGQtYmIzYi01OTMzN2M5NjczZDEiLCJhY3IiOiIxIiwic2NvcGUiOiJwcm9maWxlIGVtYWlsIn0.btjgRRZpXMSUGO9cEVBtetHFceYEeAfaKsuyirQ9mjG1UA9Ov9m5DDYJPP6vNB1mUBt805F6ugfXSPX0XBaAfJAWWdQtOk4gLTk0z7_of3cbm8QRE6x-WsE4ucl66lerSKlSNglaDb3gABGPqRp_o8eDmCjkFQH5_JBsoLUmc8t3GUkUyPMYRWap_zpy8nr0RSWIYvDSiwPyLKSQ9hiy7OxkU7USinQap5N6SlVGSenc4frfiIPwasVEncrSOeT75RakmOTgxyAilldShMXpWnJJIRkWmGnYEYglHOLA6sfprxwES4Qc5SK4exW_oDTqktDICtguZ68XXsVUs4JEbawRvc6zTlYqlo3oQMoEvDY0Aq35oJ3Nutq93W7oVOM4BN2d5idQmbCSkWBmcnriLOawP7TLQ8plhxs1v83c-M3A7SgqWo5kuk9b7PUn1TtXoDQokrG50EUO9MycmdPj0XyCTK-zKIgsr-Oy4byMrtREiqWzvz8XKzsE6nIwKfmGcQGukvmkNWSepC67CNHC0oGV5kEwo1Y7hR6f_3e26Lj-LuXzZL0D1Nx65qZJCa4Wu9rrn-F3afWqr9-ozG2gLmzYqLfKoLOhAAmy5NOJg9cqOARgqBJ1NZOzI64Mpybvei6AoXUKuXXHzvKadlhpj3Wrz0pAj2NdQbYzUPy6iGKA";

        let (header, payload, signature) = split_into_parts(input);

        assert_eq!(header, "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCIsImtpZCI6ImM5NmRjODU4NTU2NDA5MWY0Y2NiMTUzOTQ4MGRkNjlmIn0");
        assert_eq!(payload, "eyJleHAiOjE2NzM1NTk3ODQsImlhdCI6MTY3MzU1Nzk4NCwianRpIjoiNWE2ZGJhOTItNzI5YS00OWFlLTkyYzgtOWI0MDQ3NjkwMTVjIiwiaXNzIjoiaHR0cHM6Ly9hdXRoLmludC5tY2tpbnNleS5pZC9hdXRoL3JlYWxtcy9yIiwiYXVkIjoiaHR0cHM6Ly9zZG9sdXRpb25zLm1ja2luc2V5LmNvbS9taWQtaW50cy9hcGkiLCJzdWIiOiJjMzg2YWI3OC1hNjgwLTRlNjMtOGQwZi0wNmIyN2NmODkyMTkiLCJ0eXAiOiJCZWFyZXIiLCJhenAiOiI2ZGIzNGI0YS1kYjYzLTQwOGQtYmIzYi01OTMzN2M5NjczZDEiLCJhY3IiOiIxIiwic2NvcGUiOiJwcm9maWxlIGVtYWlsIn0");
        assert_eq!(signature, "btjgRRZpXMSUGO9cEVBtetHFceYEeAfaKsuyirQ9mjG1UA9Ov9m5DDYJPP6vNB1mUBt805F6ugfXSPX0XBaAfJAWWdQtOk4gLTk0z7_of3cbm8QRE6x-WsE4ucl66lerSKlSNglaDb3gABGPqRp_o8eDmCjkFQH5_JBsoLUmc8t3GUkUyPMYRWap_zpy8nr0RSWIYvDSiwPyLKSQ9hiy7OxkU7USinQap5N6SlVGSenc4frfiIPwasVEncrSOeT75RakmOTgxyAilldShMXpWnJJIRkWmGnYEYglHOLA6sfprxwES4Qc5SK4exW_oDTqktDICtguZ68XXsVUs4JEbawRvc6zTlYqlo3oQMoEvDY0Aq35oJ3Nutq93W7oVOM4BN2d5idQmbCSkWBmcnriLOawP7TLQ8plhxs1v83c-M3A7SgqWo5kuk9b7PUn1TtXoDQokrG50EUO9MycmdPj0XyCTK-zKIgsr-Oy4byMrtREiqWzvz8XKzsE6nIwKfmGcQGukvmkNWSepC67CNHC0oGV5kEwo1Y7hR6f_3e26Lj-LuXzZL0D1Nx65qZJCa4Wu9rrn-F3afWqr9-ozG2gLmzYqLfKoLOhAAmy5NOJg9cqOARgqBJ1NZOzI64Mpybvei6AoXUKuXXHzvKadlhpj3Wrz0pAj2NdQbYzUPy6iGKA");
    }

    #[test]
    fn test_decode_header() {
        let input = "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCIsImtpZCI6ImM5NmRjODU4NTU2NDA5MWY0Y2NiMTUzOTQ4MGRkNjlmIn0.eyJleHAiOjE2NzM1NTk3ODQsImlhdCI6MTY3MzU1Nzk4NCwianRpIjoiNWE2ZGJhOTItNzI5YS00OWFlLTkyYzgtOWI0MDQ3NjkwMTVjIiwiaXNzIjoiaHR0cHM6Ly9hdXRoLmludC5tY2tpbnNleS5pZC9hdXRoL3JlYWxtcy9yIiwiYXVkIjoiaHR0cHM6Ly9zZG9sdXRpb25zLm1ja2luc2V5LmNvbS9taWQtaW50cy9hcGkiLCJzdWIiOiJjMzg2YWI3OC1hNjgwLTRlNjMtOGQwZi0wNmIyN2NmODkyMTkiLCJ0eXAiOiJCZWFyZXIiLCJhenAiOiI2ZGIzNGI0YS1kYjYzLTQwOGQtYmIzYi01OTMzN2M5NjczZDEiLCJhY3IiOiIxIiwic2NvcGUiOiJwcm9maWxlIGVtYWlsIn0.btjgRRZpXMSUGO9cEVBtetHFceYEeAfaKsuyirQ9mjG1UA9Ov9m5DDYJPP6vNB1mUBt805F6ugfXSPX0XBaAfJAWWdQtOk4gLTk0z7_of3cbm8QRE6x-WsE4ucl66lerSKlSNglaDb3gABGPqRp_o8eDmCjkFQH5_JBsoLUmc8t3GUkUyPMYRWap_zpy8nr0RSWIYvDSiwPyLKSQ9hiy7OxkU7USinQap5N6SlVGSenc4frfiIPwasVEncrSOeT75RakmOTgxyAilldShMXpWnJJIRkWmGnYEYglHOLA6sfprxwES4Qc5SK4exW_oDTqktDICtguZ68XXsVUs4JEbawRvc6zTlYqlo3oQMoEvDY0Aq35oJ3Nutq93W7oVOM4BN2d5idQmbCSkWBmcnriLOawP7TLQ8plhxs1v83c-M3A7SgqWo5kuk9b7PUn1TtXoDQokrG50EUO9MycmdPj0XyCTK-zKIgsr-Oy4byMrtREiqWzvz8XKzsE6nIwKfmGcQGukvmkNWSepC67CNHC0oGV5kEwo1Y7hR6f_3e26Lj-LuXzZL0D1Nx65qZJCa4Wu9rrn-F3afWqr9-ozG2gLmzYqLfKoLOhAAmy5NOJg9cqOARgqBJ1NZOzI64Mpybvei6AoXUKuXXHzvKadlhpj3Wrz0pAj2NdQbYzUPy6iGKA";

        let (header, _payload, _signature) = split_into_parts(input);

        let decoded_1 = decode(header);

        assert!(decoded_1.is_ok());
        assert_eq!(
            decoded_1.unwrap(),
            json!({
                "alg": "RS256",
                "kid": "c96dc8585564091f4ccb1539480dd69f",
                "typ": "JWT"
            })
        );
    }

    #[test]
    fn test_decode_payload() {
        let input = "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCIsImtpZCI6ImM5NmRjODU4NTU2NDA5MWY0Y2NiMTUzOTQ4MGRkNjlmIn0.eyJleHAiOjE2NzM1NTk3ODQsImlhdCI6MTY3MzU1Nzk4NCwianRpIjoiNWE2ZGJhOTItNzI5YS00OWFlLTkyYzgtOWI0MDQ3NjkwMTVjIiwiaXNzIjoiaHR0cHM6Ly9hdXRoLmlkZW50aXR5LXByb2ZpZGVyLmlkL2F1dGgvcmVhbG1zL3IiLCJhdWQiOiJodHRwczovL2FwcGxpY2F0aW9uLmNvbSIsInN1YiI6ImMzODZhYjc4LWE2ODAtNGU2My04ZDBmLTA2YjI3Y2Y4OTIxOSIsInR5cCI6IkJlYXJlciIsImF6cCI6IjZkYjM0YjRhLWRiNjMtNDA4ZC1iYjNiLTU5MzM3Yzk2NzNkMSIsImFjciI6IjEiLCJzY29wZSI6InByb2ZpbGUgZW1haWwifQ.tjgRRZpXMSUGO9cEVBtetHFceYEeAfaKsuyirQ9mjG1UA9Ov9m5DDYJPP6vNB1mUBt805F6ugfXSPX0XBaAfJAWWdQtOk4gLTk0z7_of3cbm8QRE6x-WsE4ucl66lerSKlSNglaDb3gABGPqRp_o8eDmCjkFQH5_JBsoLUmc8t3GUkUyPMYRWap_zpy8nr0RSWIYvDSiwPyLKSQ9hiy7OxkU7USinQap5N6SlVGSenc4frfiIPwasVEncrSOeT75RakmOTgxyAilldShMXpWnJJIRkWmGnYEYglHOLA6sfprxwES4Qc5SK4exW_oDTqktDICtguZ68XXsVUs4JEbaw";

        let (_header, payload, _signature) = split_into_parts(input);

        let decoded_payload = decode(payload);

        assert!(decoded_payload.is_ok());

        assert_eq!(
            decoded_payload.unwrap(),
            json!({
                "exp": 1_673_559_784,
                "iat": 1_673_557_984,
                "jti": "5a6dba92-729a-49ae-92c8-9b404769015c",
                "iss": "https://auth.identity-profider.id/auth/realms/r",
                "aud": "https://application.com",
                "sub": "c386ab78-a680-4e63-8d0f-06b27cf89219",
                "typ": "Bearer",
                "azp": "6db34b4a-db63-408d-bb3b-59337c9673d1",
                "acr": "1",
                "scope": "profile email"
            })
        );
    }

    #[test]
    fn test_decode_signaturej() {
        let input = "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCIsImtpZCI6ImM5NmRjODU4NTU2NDA5MWY0Y2NiMTUzOTQ4MGRkNjlmIn0.eyJleHAiOjE2NzM1NTk3ODQsImlhdCI6MTY3MzU1Nzk4NCwianRpIjoiNWE2ZGJhOTItNzI5YS00OWFlLTkyYzgtOWI0MDQ3NjkwMTVjIiwiaXNzIjoiaHR0cHM6Ly9hdXRoLmlkZW50aXR5LXByb2ZpZGVyLmlkL2F1dGgvcmVhbG1zL3IiLCJhdWQiOiJodHRwczovL2FwcGxpY2F0aW9uLmNvbSIsInN1YiI6ImMzODZhYjc4LWE2ODAtNGU2My04ZDBmLTA2YjI3Y2Y4OTIxOSIsInR5cCI6IkJlYXJlciIsImF6cCI6IjZkYjM0YjRhLWRiNjMtNDA4ZC1iYjNiLTU5MzM3Yzk2NzNkMSIsImFjciI6IjEiLCJzY29wZSI6InByb2ZpbGUgZW1haWwifQ.tjgRRZpXMSUGO9cEVBtetHFceYEeAfaKsuyirQ9mjG1UA9Ov9m5DDYJPP6vNB1mUBt805F6ugfXSPX0XBaAfJAWWdQtOk4gLTk0z7_of3cbm8QRE6x-WsE4ucl66lerSKlSNglaDb3gABGPqRp_o8eDmCjkFQH5_JBsoLUmc8t3GUkUyPMYRWap_zpy8nr0RSWIYvDSiwPyLKSQ9hiy7OxkU7USinQap5N6SlVGSenc4frfiIPwasVEncrSOeT75RakmOTgxyAilldShMXpWnJJIRkWmGnYEYglHOLA6sfprxwES4Qc5SK4exW_oDTqktDICtguZ68XXsVUs4JEbaw";

        let (_header, _payload, _signature) = split_into_parts(input);

        // TODO
        // let decoded_payload = decode(payload);

        // assert!(decoded_payload.is_ok());

        // assert_eq!(
        //     decoded_payload.unwrap(),
        //     json!({
        //         "exp": 1_673_559_784,
        //         "iat": 1_673_557_984,
        //         "jti": "5a6dba92-729a-49ae-92c8-9b404769015c",
        //         "iss": "https://auth.identity-profider.id/auth/realms/r",
        //         "aud": "https://application.com",
        //         "sub": "c386ab78-a680-4e63-8d0f-06b27cf89219",
        //         "typ": "Bearer",
        //         "azp": "6db34b4a-db63-408d-bb3b-59337c9673d1",
        //         "acr": "1",
        //         "scope": "profile email"
        //     })
        // );
    }
}
