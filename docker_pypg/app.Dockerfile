FROM python:3.12-slim as base

ARG MY_APP

ENV MY_APP=${MY_APP} \
    PYTHONFAULTHANDLER=1 \
    PYTHONHASHSEED=random \
    PYTHONUNBUFFERED=1

RUN apt update && apt install -y gcc python3-dev libpq-dev #libffi6

WORKDIR /app

FROM base as builder

ENV PIP_DEFAULT_TIMEOUT=100 \
    PIP_DISABLE_PIP_VERSION_CHECK=1 \
    PIP_NO_CACHE_DIR=1 \
    POETRY_VERSION=1.3.1

RUN pip install --upgrade pip
RUN pip install "poetry==$POETRY_VERSION"

# COPY pyproject.toml poetry.lock README.md ./
COPY "$MY_APP" ./
RUN poetry config virtualenvs.in-project true && \
    poetry install --only=main --no-root
RUN poetry build
#RUN . /venv/bin/activate && poetry install --no-root $(test "$YOUR_ENV" == production && echo "--no-dev")

FROM base as final

COPY --from=builder /app/.venv ./.venv
COPY --from=builder /app/dist .
RUN pip install *.whl

EXPOSE 8000
CMD ["pypg_server"]
