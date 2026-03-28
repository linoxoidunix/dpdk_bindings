#!/bin/bash
set -e

# ==========================================
# Настройки версии (Источник истины)
# ==========================================
DPDK_VER="25.11"
# ==========================================

# 1. Настройка путей
# $1 - первый аргумент (INSTALL_DIR), $2 - второй аргумент (BUILD_TMP)
# Если аргументы не переданы, используем дефолтные папки в текущей директории
INSTALL_DIR="${1:-$(pwd)/install}"
BUILD_TMP="${2:-$(pwd)/build_tmp}"

DPDK_DIR="dpdk-$DPDK_VER"
DPDK_TAR="$DPDK_DIR.tar.xz"

echo "--- Начинаем сборку DPDK $DPDK_VER ---"
echo "Путь установки: $INSTALL_DIR"
echo "Временная папка сборки: $BUILD_TMP"

# 2. Создаем директории
mkdir -p "$BUILD_TMP"
mkdir -p "$INSTALL_DIR"

# Переходим во временную папку
pushd "$BUILD_TMP" > /dev/null

# 3. Скачивание исходников
if [ ! -f "$DPDK_TAR" ]; then
    echo "Скачивание $DPDK_TAR..."
    wget -c "https://fast.dpdk.org/rel/$DPDK_TAR"
fi

# 4. Распаковка
echo "Распаковка $DPDK_DIR..."
# Используем --strip-components=1, чтобы распаковать содержимое сразу в текущую папку
# Это чище, чем cd внутрь распакованной папки
tar -xJf "$DPDK_TAR" --strip-components=1

# 5. Подготовка инструментов
echo "Проверка инструментов сборки..."
pip3 install --upgrade pyelftools meson ninja

# 6. Сборка
echo "Конфигурация Meson..."
# CC/CXX можно оставить жесткими, если уверен, что gcc-14 есть в системе
CC=gcc-14 CXX=g++-14 meson setup build \
    --prefix="$INSTALL_DIR" \
    --libdir="lib64" \
    -Ddisable_drivers=net/gve,net/ionic \
    -Dbuildtype=release \
    -Ddefault_library=shared

echo "Компиляция..."
ninja -C build

echo "Установка в $INSTALL_DIR..."
ninja -C build install

# 7. Возвращаемся
popd > /dev/null

echo "--- Сборка $DPDK_VER завершена успешно! ---"
echo "Библиотеки: $INSTALL_DIR/lib64"
echo "Инклюды: $INSTALL_DIR/include"