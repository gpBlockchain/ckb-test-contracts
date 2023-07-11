import requests
from lxml import html



def check_contains_feature(content):
    return "feature" in content

def remove_use(content):
    lines = content.split("\n")
    filtered_lines = [line for line in lines if not line.startswith("use std::sync::atomic")]
    return "\n".join(filtered_lines)

# 发起GET请求获取网页内容
url = "https://doc.rust-lang.org/stable/core/sync/atomic/struct.AtomicU64.html"
response = requests.get(url)
html_content = response.text

# 使用lxml解析网页内容并提取匹配的元素
parsed_html = html.fromstring(html_content)
elements = parsed_html.xpath("//pre[@class='rust rust-example-rendered']")

# 提取元素内容
for element in elements:
    content = element.text_content()
    if check_contains_feature(content):
        continue
    content = remove_use(content)
    print(content)
