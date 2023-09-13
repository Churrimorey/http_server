namespace rs redis.base

enum RequestType{
    Get,
    Set,
    Del,
    Ping,
    Publish,
    Subscribe,
    Exit,
    Illegal
}


struct RedisRequest{
    1: optional list<string> key,
    2: optional string value,
    3: required RequestType type,
}

enum ResponseType{
    Value,
    Ok,
    Trap,
}

struct RedisResponse{
    1: optional string value,
    2: required ResponseType type,
}

service RedisService{
    RedisResponse RedisCommand (1: RedisRequest req),
} 
