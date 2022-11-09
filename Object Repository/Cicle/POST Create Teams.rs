<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>POST Create Teams</name>
   <tag></tag>
   <elementGuidId>c2e83d46-6303-463c-88b0-c3f4a3eb5e03</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;data\&quot;: {\n        \&quot;desc\&quot;: \&quot;Saya Coba Nih\&quot;,\n        \&quot;name\&quot;: \&quot;Malaysia\&quot;,\n        \&quot;type\&quot;: \&quot;team\&quot;\n    },\n    \&quot;selector\&quot;: {\n        \&quot;companyId\&quot;: \&quot;61eba2c85080f47b25ddc8f8\&quot;\n    }\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>80ee3526-cd8d-4ef0-bc24-d7e3260a9159</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>jwt eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VyIjp7Il9pZCI6IjYzNTAwMGJjMmUyMGQ4OGIyOGI2MWMwMCIsImdvb2dsZUlkIjoiMTAzMzMzNjA3NDgwNjgyNjc3NTI5IiwiZW1haWwiOiJkd2ltYWhlbmRyYW5hbmRhQGdtYWlsLmNvbSIsImZ1bGxOYW1lIjoiTmFuZGEgRHdpIE1haGVuZHJhIiwicGhvdG9VcmwiOiJodHRwczovL2xoMy5nb29nbGV1c2VyY29udGVudC5jb20vYS9BTG01d3UwLUxsV3JYamNzQ0RrRVFpWWV0WUp2c19NTWh6RUplUHI4M3V5TnhBPXM5Ni1jIiwiYmlvIjoiIiwic3RhdHVzIjoiIiwiZGVmYXVsdENvbXBhbnkiOnsiX2lkIjoiNjFlYmEyYzg1MDgwZjQ3YjI1ZGRjOGY4In0sImNyZWF0ZWRBdCI6IjIwMjItMTAtMTlUMTM6NTA6NTIuODU2WiIsInVwZGF0ZWRBdCI6IjIwMjItMTAtMzFUMTQ6Mzk6NDcuMTk1WiIsIl9fdiI6MH0sImlhdCI6MTY2NzU2NjgwMSwiZXhwIjoxNjcwMTU4ODAxfQ.34YW-r3VWMzNetALPO5DfCBV-sUmrXNL5Rz390axdiA</value>
      <webElementGuid>08a45f8c-77d0-48d0-9296-1741e99df353</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.4.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://stagingapi.cicle.app/api/v1/teams</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
