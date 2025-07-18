use base64::Engine as _;
use base64::engine::general_purpose;
use color_eyre::eyre::Report;

fn main() -> Result<(), color_eyre::Report> {
    color_eyre::install()?;

    let argument = if let Some(arg) = std::env::args().nth(1) {
        arg
    } else {
        let mut buffer = String::new();

        let stdin = std::io::stdin();
        stdin.read_line(&mut buffer)?;

        buffer
    };

    if argument.is_empty() {
        return Err(Report::msg("No JWT in STDIN / argument."));
    }

    // sometimes people pass in "a.b.c", for example from jq. They can use -r,
    // but we're nice people, so why not help them?
    let argument = if argument.starts_with('"') && argument.ends_with('"') {
        argument.chars().skip(1).take(argument.len() - 2).collect()
    } else {
        argument
    };

    let (header, payload, _signature) = split_into_parts(&argument)?;

    let part_1 = decode(header)?;
    pretty_print(&part_1)?;

    let part_2 = decode(payload)?;
    pretty_print(&part_2)?;

    Ok(())
}

fn split_into_parts(jwt: &str) -> Result<(&str, &str, &str), color_eyre::Report> {
    let split = jwt.split('.').filter(|p| !p.is_empty()).collect::<Vec<_>>();

    if let &[a, b, c] = split.as_slice() {
        Ok((a, b, c))
    } else {
        Err(Report::msg(format!(
            "Invalid JWT, need a.b.c and you have {} pieces.",
            split.len(),
        )))
    }
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
    use color_eyre::eyre;
    use serde_json::json;

    use crate::{decode, split_into_parts};

    #[test]
    fn split() {
        let input = "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCIsImtpZCI6ImM5NmRjODU4NTU2NDA5MWY0Y2NiMTUzOTQ4MGRkNjlmIn0.eyJleHAiOjE2NzM1NTk3ODQsImlhdCI6MTY3MzU1Nzk4NCwianRpIjoiNWE2ZGJhOTItNzI5YS00OWFlLTkyYzgtOWI0MDQ3NjkwMTVjIiwiaXNzIjoiaHR0cHM6Ly9hdXRoLmludC5tY2tpbnNleS5pZC9hdXRoL3JlYWxtcy9yIiwiYXVkIjoiaHR0cHM6Ly9zZG9sdXRpb25zLm1ja2luc2V5LmNvbS9taWQtaW50cy9hcGkiLCJzdWIiOiJjMzg2YWI3OC1hNjgwLTRlNjMtOGQwZi0wNmIyN2NmODkyMTkiLCJ0eXAiOiJCZWFyZXIiLCJhenAiOiI2ZGIzNGI0YS1kYjYzLTQwOGQtYmIzYi01OTMzN2M5NjczZDEiLCJhY3IiOiIxIiwic2NvcGUiOiJwcm9maWxlIGVtYWlsIn0.btjgRRZpXMSUGO9cEVBtetHFceYEeAfaKsuyirQ9mjG1UA9Ov9m5DDYJPP6vNB1mUBt805F6ugfXSPX0XBaAfJAWWdQtOk4gLTk0z7_of3cbm8QRE6x-WsE4ucl66lerSKlSNglaDb3gABGPqRp_o8eDmCjkFQH5_JBsoLUmc8t3GUkUyPMYRWap_zpy8nr0RSWIYvDSiwPyLKSQ9hiy7OxkU7USinQap5N6SlVGSenc4frfiIPwasVEncrSOeT75RakmOTgxyAilldShMXpWnJJIRkWmGnYEYglHOLA6sfprxwES4Qc5SK4exW_oDTqktDICtguZ68XXsVUs4JEbawRvc6zTlYqlo3oQMoEvDY0Aq35oJ3Nutq93W7oVOM4BN2d5idQmbCSkWBmcnriLOawP7TLQ8plhxs1v83c-M3A7SgqWo5kuk9b7PUn1TtXoDQokrG50EUO9MycmdPj0XyCTK-zKIgsr-Oy4byMrtREiqWzvz8XKzsE6nIwKfmGcQGukvmkNWSepC67CNHC0oGV5kEwo1Y7hR6f_3e26Lj-LuXzZL0D1Nx65qZJCa4Wu9rrn-F3afWqr9-ozG2gLmzYqLfKoLOhAAmy5NOJg9cqOARgqBJ1NZOzI64Mpybvei6AoXUKuXXHzvKadlhpj3Wrz0pAj2NdQbYzUPy6iGKA";

        let (header, payload, signature) = split_into_parts(input).expect("Invalid JWT");

        assert_eq!(
            header,
            "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCIsImtpZCI6ImM5NmRjODU4NTU2NDA5MWY0Y2NiMTUzOTQ4MGRkNjlmIn0"
        );
        assert_eq!(
            payload,
            "eyJleHAiOjE2NzM1NTk3ODQsImlhdCI6MTY3MzU1Nzk4NCwianRpIjoiNWE2ZGJhOTItNzI5YS00OWFlLTkyYzgtOWI0MDQ3NjkwMTVjIiwiaXNzIjoiaHR0cHM6Ly9hdXRoLmludC5tY2tpbnNleS5pZC9hdXRoL3JlYWxtcy9yIiwiYXVkIjoiaHR0cHM6Ly9zZG9sdXRpb25zLm1ja2luc2V5LmNvbS9taWQtaW50cy9hcGkiLCJzdWIiOiJjMzg2YWI3OC1hNjgwLTRlNjMtOGQwZi0wNmIyN2NmODkyMTkiLCJ0eXAiOiJCZWFyZXIiLCJhenAiOiI2ZGIzNGI0YS1kYjYzLTQwOGQtYmIzYi01OTMzN2M5NjczZDEiLCJhY3IiOiIxIiwic2NvcGUiOiJwcm9maWxlIGVtYWlsIn0"
        );
        assert_eq!(
            signature,
            "btjgRRZpXMSUGO9cEVBtetHFceYEeAfaKsuyirQ9mjG1UA9Ov9m5DDYJPP6vNB1mUBt805F6ugfXSPX0XBaAfJAWWdQtOk4gLTk0z7_of3cbm8QRE6x-WsE4ucl66lerSKlSNglaDb3gABGPqRp_o8eDmCjkFQH5_JBsoLUmc8t3GUkUyPMYRWap_zpy8nr0RSWIYvDSiwPyLKSQ9hiy7OxkU7USinQap5N6SlVGSenc4frfiIPwasVEncrSOeT75RakmOTgxyAilldShMXpWnJJIRkWmGnYEYglHOLA6sfprxwES4Qc5SK4exW_oDTqktDICtguZ68XXsVUs4JEbawRvc6zTlYqlo3oQMoEvDY0Aq35oJ3Nutq93W7oVOM4BN2d5idQmbCSkWBmcnriLOawP7TLQ8plhxs1v83c-M3A7SgqWo5kuk9b7PUn1TtXoDQokrG50EUO9MycmdPj0XyCTK-zKIgsr-Oy4byMrtREiqWzvz8XKzsE6nIwKfmGcQGukvmkNWSepC67CNHC0oGV5kEwo1Y7hR6f_3e26Lj-LuXzZL0D1Nx65qZJCa4Wu9rrn-F3afWqr9-ozG2gLmzYqLfKoLOhAAmy5NOJg9cqOARgqBJ1NZOzI64Mpybvei6AoXUKuXXHzvKadlhpj3Wrz0pAj2NdQbYzUPy6iGKA"
        );
    }

    #[test]
    fn decode_header() {
        let input = "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCIsImtpZCI6ImM5NmRjODU4NTU2NDA5MWY0Y2NiMTUzOTQ4MGRkNjlmIn0.eyJleHAiOjE2NzM1NTk3ODQsImlhdCI6MTY3MzU1Nzk4NCwianRpIjoiNWE2ZGJhOTItNzI5YS00OWFlLTkyYzgtOWI0MDQ3NjkwMTVjIiwiaXNzIjoiaHR0cHM6Ly9hdXRoLmludC5tY2tpbnNleS5pZC9hdXRoL3JlYWxtcy9yIiwiYXVkIjoiaHR0cHM6Ly9zZG9sdXRpb25zLm1ja2luc2V5LmNvbS9taWQtaW50cy9hcGkiLCJzdWIiOiJjMzg2YWI3OC1hNjgwLTRlNjMtOGQwZi0wNmIyN2NmODkyMTkiLCJ0eXAiOiJCZWFyZXIiLCJhenAiOiI2ZGIzNGI0YS1kYjYzLTQwOGQtYmIzYi01OTMzN2M5NjczZDEiLCJhY3IiOiIxIiwic2NvcGUiOiJwcm9maWxlIGVtYWlsIn0.btjgRRZpXMSUGO9cEVBtetHFceYEeAfaKsuyirQ9mjG1UA9Ov9m5DDYJPP6vNB1mUBt805F6ugfXSPX0XBaAfJAWWdQtOk4gLTk0z7_of3cbm8QRE6x-WsE4ucl66lerSKlSNglaDb3gABGPqRp_o8eDmCjkFQH5_JBsoLUmc8t3GUkUyPMYRWap_zpy8nr0RSWIYvDSiwPyLKSQ9hiy7OxkU7USinQap5N6SlVGSenc4frfiIPwasVEncrSOeT75RakmOTgxyAilldShMXpWnJJIRkWmGnYEYglHOLA6sfprxwES4Qc5SK4exW_oDTqktDICtguZ68XXsVUs4JEbawRvc6zTlYqlo3oQMoEvDY0Aq35oJ3Nutq93W7oVOM4BN2d5idQmbCSkWBmcnriLOawP7TLQ8plhxs1v83c-M3A7SgqWo5kuk9b7PUn1TtXoDQokrG50EUO9MycmdPj0XyCTK-zKIgsr-Oy4byMrtREiqWzvz8XKzsE6nIwKfmGcQGukvmkNWSepC67CNHC0oGV5kEwo1Y7hR6f_3e26Lj-LuXzZL0D1Nx65qZJCa4Wu9rrn-F3afWqr9-ozG2gLmzYqLfKoLOhAAmy5NOJg9cqOARgqBJ1NZOzI64Mpybvei6AoXUKuXXHzvKadlhpj3Wrz0pAj2NdQbYzUPy6iGKA";

        let (header, _payload, _signature) = split_into_parts(input).expect("Invalid JWT");

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
    fn decode_payload() {
        let input = "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCIsImtpZCI6ImM5NmRjODU4NTU2NDA5MWY0Y2NiMTUzOTQ4MGRkNjlmIn0.eyJleHAiOjE2NzM1NTk3ODQsImlhdCI6MTY3MzU1Nzk4NCwianRpIjoiNWE2ZGJhOTItNzI5YS00OWFlLTkyYzgtOWI0MDQ3NjkwMTVjIiwiaXNzIjoiaHR0cHM6Ly9hdXRoLmlkZW50aXR5LXByb2ZpZGVyLmlkL2F1dGgvcmVhbG1zL3IiLCJhdWQiOiJodHRwczovL2FwcGxpY2F0aW9uLmNvbSIsInN1YiI6ImMzODZhYjc4LWE2ODAtNGU2My04ZDBmLTA2YjI3Y2Y4OTIxOSIsInR5cCI6IkJlYXJlciIsImF6cCI6IjZkYjM0YjRhLWRiNjMtNDA4ZC1iYjNiLTU5MzM3Yzk2NzNkMSIsImFjciI6IjEiLCJzY29wZSI6InByb2ZpbGUgZW1haWwifQ.tjgRRZpXMSUGO9cEVBtetHFceYEeAfaKsuyirQ9mjG1UA9Ov9m5DDYJPP6vNB1mUBt805F6ugfXSPX0XBaAfJAWWdQtOk4gLTk0z7_of3cbm8QRE6x-WsE4ucl66lerSKlSNglaDb3gABGPqRp_o8eDmCjkFQH5_JBsoLUmc8t3GUkUyPMYRWap_zpy8nr0RSWIYvDSiwPyLKSQ9hiy7OxkU7USinQap5N6SlVGSenc4frfiIPwasVEncrSOeT75RakmOTgxyAilldShMXpWnJJIRkWmGnYEYglHOLA6sfprxwES4Qc5SK4exW_oDTqktDICtguZ68XXsVUs4JEbaw";

        let (_header, payload, _signature) = split_into_parts(input).expect("Invalid JWT");

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
    fn invalid_jwt() {
        let inputs = [
            "a", ".", "a.b", "a.b.", "a.b..", ".a.b", "..a.b", ".a.b.", "..a.b..",
        ];

        for input in inputs {
            let r = split_into_parts(input);

            let _r: eyre::Report = r.unwrap_err();
        }
    }

    #[test]
    fn decode_signature() {
        let input = "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCIsImtpZCI6ImM5NmRjODU4NTU2NDA5MWY0Y2NiMTUzOTQ4MGRkNjlmIn0.eyJleHAiOjE2NzM1NTk3ODQsImlhdCI6MTY3MzU1Nzk4NCwianRpIjoiNWE2ZGJhOTItNzI5YS00OWFlLTkyYzgtOWI0MDQ3NjkwMTVjIiwiaXNzIjoiaHR0cHM6Ly9hdXRoLmlkZW50aXR5LXByb2ZpZGVyLmlkL2F1dGgvcmVhbG1zL3IiLCJhdWQiOiJodHRwczovL2FwcGxpY2F0aW9uLmNvbSIsInN1YiI6ImMzODZhYjc4LWE2ODAtNGU2My04ZDBmLTA2YjI3Y2Y4OTIxOSIsInR5cCI6IkJlYXJlciIsImF6cCI6IjZkYjM0YjRhLWRiNjMtNDA4ZC1iYjNiLTU5MzM3Yzk2NzNkMSIsImFjciI6IjEiLCJzY29wZSI6InByb2ZpbGUgZW1haWwifQ.tjgRRZpXMSUGO9cEVBtetHFceYEeAfaKsuyirQ9mjG1UA9Ov9m5DDYJPP6vNB1mUBt805F6ugfXSPX0XBaAfJAWWdQtOk4gLTk0z7_of3cbm8QRE6x-WsE4ucl66lerSKlSNglaDb3gABGPqRp_o8eDmCjkFQH5_JBsoLUmc8t3GUkUyPMYRWap_zpy8nr0RSWIYvDSiwPyLKSQ9hiy7OxkU7USinQap5N6SlVGSenc4frfiIPwasVEncrSOeT75RakmOTgxyAilldShMXpWnJJIRkWmGnYEYglHOLA6sfprxwES4Qc5SK4exW_oDTqktDICtguZ68XXsVUs4JEbaw";

        let (_header, _payload, _signature) = split_into_parts(input).expect("Invalid JWT");

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
