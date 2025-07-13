import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export interface ServiceConfig {
  server: {
    host: string
    port: number
  }
  learning: {
    model_path: string
    log_level: string
  }
  payments: {
    stripe_enabled: boolean
    crypto_enabled: boolean
  }
}

export interface AnalysisRequest {
  content_type: string
  content: string
  session_id: string
}

export interface PaymentRequest {
  amount: number
  currency: string
  description: string
  customer_email: string
  payment_method: string
  crypto_details?: {
    currency: string
    network: string
    wallet_address: string
  }
}

export interface StreamRequest {
  session_id: string
  buffer_size?: number
  sample_rate?: number
}

export class ServiceBridgeError extends Error {
  constructor(message: string, public code?: string) {
    super(message)
    this.name = 'ServiceBridgeError'
  }
}

export function useServiceBridge() {
  const isServiceAvailable = ref(false)
  const isLoading = ref(false)
  const lastError = ref<string | null>(null)

  // Check service health
  const checkServiceHealth = async () => {
    try {
      isLoading.value = true
      lastError.value = null
      
      const response = await invoke('check_service_health')
      isServiceAvailable.value = true
      return response
    } catch (error) {
      isServiceAvailable.value = false
      lastError.value = error as string
      throw new ServiceBridgeError(`Service health check failed: ${error}`)
    } finally {
      isLoading.value = false
    }
  }

  // Get service status
  const getServiceStatus = async () => {
    try {
      isLoading.value = true
      lastError.value = null
      
      const response = await invoke('get_service_status')
      return response
    } catch (error) {
      lastError.value = error as string
      throw new ServiceBridgeError(`Failed to get service status: ${error}`)
    } finally {
      isLoading.value = false
    }
  }

  // ===== LEARNING OPERATIONS =====

  // Get AI models
  const getAIModels = async () => {
    try {
      isLoading.value = true
      lastError.value = null
      
      const response = await invoke('get_ai_models')
      return response
    } catch (error) {
      lastError.value = error as string
      throw new ServiceBridgeError(`Failed to get AI models: ${error}`)
    } finally {
      isLoading.value = false
    }
  }

  // Analyze content (unified)
  const analyzeContent = async (request: AnalysisRequest) => {
    try {
      isLoading.value = true
      lastError.value = null
      
      const response = await invoke('unified_analyze_content', {
        content_type: request.content_type,
        content: request.content,
        session_id: request.session_id
      })
      return response
    } catch (error) {
      lastError.value = error as string
      throw new ServiceBridgeError(`Content analysis failed: ${error}`)
    } finally {
      isLoading.value = false
    }
  }

  // Process OCR (unified)
  const processOCR = async (imagePath: string, sessionId: string) => {
    try {
      isLoading.value = true
      lastError.value = null
      
      const response = await invoke('unified_process_ocr', {
        image_path: imagePath,
        session_id: sessionId
      })
      return response
    } catch (error) {
      lastError.value = error as string
      throw new ServiceBridgeError(`OCR processing failed: ${error}`)
    } finally {
      isLoading.value = false
    }
  }

  // ===== PAYMENT OPERATIONS =====

  // Get payment methods
  const getPaymentMethods = async () => {
    try {
      isLoading.value = true
      lastError.value = null
      
      const response = await invoke('get_payment_methods')
      return response
    } catch (error) {
      lastError.value = error as string
      throw new ServiceBridgeError(`Failed to get payment methods: ${error}`)
    } finally {
      isLoading.value = false
    }
  }

  // Process payment (unified)
  const processPayment = async (request: PaymentRequest) => {
    try {
      isLoading.value = true
      lastError.value = null
      
      const response = await invoke('unified_process_payment', {
        amount: request.amount,
        currency: request.currency,
        description: request.description,
        customer_email: request.customer_email,
        payment_method: request.payment_method,
        crypto_details: request.crypto_details
      })
      return response
    } catch (error) {
      lastError.value = error as string
      throw new ServiceBridgeError(`Payment processing failed: ${error}`)
    } finally {
      isLoading.value = false
    }
  }

  // ===== CONFIGURATION OPERATIONS =====

  // Get configuration (unified)
  const getConfiguration = async () => {
    try {
      isLoading.value = true
      lastError.value = null
      
      const response = await invoke('unified_get_config')
      return response
    } catch (error) {
      lastError.value = error as string
      throw new ServiceBridgeError(`Configuration retrieval failed: ${error}`)
    } finally {
      isLoading.value = false
    }
  }

  // Update configuration (unified)
  const updateConfiguration = async (config: Partial<ServiceConfig>) => {
    try {
      isLoading.value = true
      lastError.value = null
      
      const response = await invoke('unified_update_config', {
        config: config
      })
      return response
    } catch (error) {
      lastError.value = error as string
      throw new ServiceBridgeError(`Configuration update failed: ${error}`)
    } finally {
      isLoading.value = false
    }
  }

  // ===== STREAM OPERATIONS =====

  // Start audio stream (unified)
  const startAudioStream = async (request: StreamRequest) => {
    try {
      isLoading.value = true
      lastError.value = null
      
      const response = await invoke('unified_start_audio_stream', {
        session_id: request.session_id,
        buffer_size: request.buffer_size,
        sample_rate: request.sample_rate
      })
      return response
    } catch (error) {
      lastError.value = error as string
      throw new ServiceBridgeError(`Audio stream start failed: ${error}`)
    } finally {
      isLoading.value = false
    }
  }

  // Stop audio stream (unified)
  const stopAudioStream = async (sessionId: string) => {
    try {
      isLoading.value = true
      lastError.value = null
      
      const response = await invoke('unified_stop_audio_stream', {
        session_id: sessionId
      })
      return response
    } catch (error) {
      lastError.value = error as string
      throw new ServiceBridgeError(`Audio stream stop failed: ${error}`)
    } finally {
      isLoading.value = false
    }
  }

  // Get stream status
  const getStreamStatus = async () => {
    try {
      isLoading.value = true
      lastError.value = null
      
      const response = await invoke('get_stream_status')
      return response
    } catch (error) {
      lastError.value = error as string
      throw new ServiceBridgeError(`Failed to get stream status: ${error}`)
    } finally {
      isLoading.value = false
    }
  }

  // ===== UTILITY FUNCTIONS =====

  // Clear error
  const clearError = () => {
    lastError.value = null
  }

  // Initialize service bridge
  const initialize = async () => {
    try {
      await checkServiceHealth()
      console.log('Service bridge initialized successfully')
    } catch (error) {
      console.warn('Service bridge initialization failed, will use fallback mode:', error)
    }
  }

  // Computed properties
  const hasError = computed(() => lastError.value !== null)
  const isReady = computed(() => !isLoading.value)

  return {
    // State
    isServiceAvailable,
    isLoading,
    lastError,
    hasError,
    isReady,

    // Health and status
    checkServiceHealth,
    getServiceStatus,

    // Learning operations
    getAIModels,
    analyzeContent,
    processOCR,

    // Payment operations
    getPaymentMethods,
    processPayment,

    // Configuration operations
    getConfiguration,
    updateConfiguration,

    // Stream operations
    startAudioStream,
    stopAudioStream,
    getStreamStatus,

    // Utility functions
    clearError,
    initialize
  }
} 