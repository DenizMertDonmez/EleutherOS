# Telif Hakkı
Copyright (C) 2026 Deniz Mert Dönmez

# EleutherOS

Yunanca "özgür" anlamına gelen eleutheros kelimesinden türetilmiştir.  
Rust ile yazılan, bare-metal ARM64 çekirdeği. QEMU ve Raspberry Pi 5 hedeflenir.

> Not: LGS sınavına hazırlandığım için projeye ara verebilirim. Anlayışınız için teşekkürler.

> Not: Geliştirmede yapay zekadan yardım alınmaktadır

> Not: RPi5 ile sorunlar oluşabilir, en yakın zamanda düzeltilecektir.

## 1. YAPILMA NEDENİ

Bilgisayar mimarisini anlama, özgür yazılıma katkı, belki bir gün iyi bir işletim sistemi.

## 2. HEDEF DONANIM

Hem QEMU'da Aarch64 ile hem Raspberry Pi 5'te çalışacak şekilde tasarlanıyor. Derleme sırasında features'ı qemu olarak belirtirseniz Qemu için, eğer bir şey belirtmezseniz Raspberry Pi 5 için derlenir.

## 3. MEVCUT ÖZELLİKLER

Şu an UART üzerinden karakter basılabiliyor, sadece çekirdek 0 çalışıyor.

## 4. ÇALIŞTIRMA

Gereksinimler: Rust 2024 ve QEMU veya Raspberry Pi 5

Komutlar: Hedef ekleme: rustup target add aarch64-unknown-none

QEMU için derleme: cargo build --features qemu

QEMU ile çalıştırma: qemu-system-aarch64 -M virt -cpu cortex-a53 -kernel target/aarch64-unknown-none/debug/eleutheros -nographic

## 5. LİSANS

GPL 3.0 ile lisanslıdır. LICENSE dosyasından daha fazlasını öğrenebilirsiniz

## 6. KATKI

Başta da belirttiğim üzere, LGS 2027 sınavına hazırlanıyorum. Bu sebeple de meşgulüm. Sınavdan sonraki yaz projeye daha çok vakit ayıracağım.

