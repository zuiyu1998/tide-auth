# tide-auth
tide框架的auth中间件。

# 功能
该框架只会对req的请求头进行解析，在错误时会返回403，并将解析的结果存储在req的ext中，不会涉及存储等知识。