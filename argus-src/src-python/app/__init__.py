# app/__init__.py
from flask import Flask
from app.routes.example import example_bp


def create_app(config_class=None):
    app = Flask(__name__)

    # 加载配置
    # if config_class:
    #     app.config.from_object(config_class)

    # 注册蓝图
    app.register_blueprint(example_bp, url_prefix='')

    return app
