FROM rust:1.64.0 as base

RUN rustup toolchain install 1.64.0-x86_64-unknown-linux-gnu && \
    rustup target add x86_64-unknown-linux-gnu armv7-linux-androideabi aarch64-linux-android i686-linux-android x86_64-linux-android && \
    apt-get upgrade && apt-get update && apt install -y wget unzip default-jdk && \
    cargo install --git=https://github.com/dodorare/crossbow crossbundle

ENV GRADLE_VERSION=7.5.1
ENV NDK_VERSION=25.1.8937393
ENV BUNDLETOOL_VERSION=1.13.2
ENV BUILDTOOLS_VERSION=31.0.0
ENV PLATFORM_VERSION=android-31
ENV ANDROID_SDK_ROOT=/opt/Android
ENV ANDROID_NDK_ROOT=/opt/Android/ndk/${NDK_VERSION}
ENV NDK_HOME=/opt/Android/ndk/${NDK_VERSION}
ENV GRADLE_HOME=/opt/gradle/gradle-${GRADLE_VERSION}/bin
ENV BUNDLETOOL_PATH=${ANDROID_SDK_ROOT}/bundletool-all-${BUNDLETOOL_VERSION}.jar
ENV PATH=$PATH:${ANDROID_SDK_ROOT}:${ANDROID_NDK_ROOT}:${GRADLE_HOME}

RUN wget -c https://services.gradle.org/distributions/gradle-${GRADLE_VERSION}-bin.zip -P /tmp && unzip -d /opt/gradle /tmp/gradle-${GRADLE_VERSION}-bin.zip

RUN mkdir ${ANDROID_SDK_ROOT} && crossbundle install command-line-tools -i ${ANDROID_SDK_ROOT} -f
RUN crossbundle install bundletool -v ${BUNDLETOOL_VERSION} -p ${ANDROID_SDK_ROOT}
RUN echo y | crossbundle install sdkmanager --install "build-tools;${BUILDTOOLS_VERSION}" "ndk;${NDK_VERSION}" "platforms;${PLATFORM_VERSION}"

ENTRYPOINT [ "crossbundle" ]
